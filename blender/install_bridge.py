"""Block's bundled, offline installer, run by the selected Blender executable.

Blender loads its preferences normally (re-reading them via an operator crashes
some 4.2 builds). Only Block LiveMesh's package/enable state is changed, and no
scene is saved. Probe never saves preferences.

The extension id is `block_live_mesh` (Blender 4.2+). Older Aurora installs
(`aurora_live_mesh` / `aurora_live`) are disabled and left alone so the new
package can enable cleanly.
"""
import ast
from contextlib import contextmanager
from datetime import datetime, timezone
import importlib
import json
import os
from pathlib import Path
import re
import shutil
import sys
import uuid
import zipfile

import bpy

# Background probes must not autosave preferences dirtied by existing add-ons.
# Installation serializes the original value, then disables autosave again for
# this disposable process, including error/rollback exits.
AUTO_SAVE = getattr(bpy.context.preferences, "use_preferences_save", None)
if AUTO_SAVE is not None:
    bpy.context.preferences.use_preferences_save = False

PKG = "block_live_mesh"
LEGACY_PKG = "block_live"
# Former Aurora package names still found in some profiles.
OLD_PKG = "aurora_live_mesh"
OLD_LEGACY = "aurora_live"


def checked(result):
    if "FINISHED" not in result:
        raise RuntimeError("Blender cancelled the operation: " + str(result))


def package_version(directory, extension):
    try:
        if extension:
            text = (directory / "blender_manifest.toml").read_text(encoding="utf-8")
            match = re.search(r'^version\s*=\s*"([^"]+)"', text, re.MULTILINE)
            return match.group(1) if match else None
        tree = ast.parse((directory / "__init__.py").read_text(encoding="utf-8"))
        for node in tree.body:
            if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "bl_info" for t in node.targets):
                return ".".join(str(v) for v in ast.literal_eval(node.value)["version"])
    except (OSError, SyntaxError, ValueError, KeyError, TypeError):
        pass
    return None


def locations():
    extension = bpy.app.version >= (4, 2, 0)
    config = Path(bpy.utils.user_resource("CONFIG"))
    overridden = os.environ.get("BLENDER_USER_CONFIG")
    if overridden and config.resolve() != Path(overridden).resolve():
        raise RuntimeError("Blender did not honor BLENDER_USER_CONFIG; refusing to touch a different profile.")
    scripts = Path(bpy.utils.user_resource("SCRIPTS")) / "addons"
    legacy = scripts / LEGACY_PKG
    repo = None
    if extension:
        repos = bpy.context.preferences.extensions.repos
        # Prefer an existing Block install; fall back to User Default.
        repo = next(
            (
                r
                for r in repos
                if r.enabled and not r.use_remote_url and (Path(r.directory) / PKG).is_dir()
            ),
            None,
        )
        if repo is None:
            repo = next((r for r in repos if r.module == "user_default" and r.enabled and not r.use_remote_url), None)
        if repo is None:
            raise RuntimeError("No enabled local extension repository. Enable User Default in Blender preferences.")
        target = Path(repo.directory) / PKG
        module = "bl_ext." + repo.module + "." + PKG
    else:
        target, module = legacy, LEGACY_PKG
    return extension, config, legacy, repo, target, module


@contextmanager
def install_lock(path):
    """The OS releases this lock even if the viewer terminates its child."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a+b") as handle:
        handle.seek(0, os.SEEK_END)
        if handle.tell() == 0:
            handle.write(b"0")
            handle.flush()
        handle.seek(0)
        if os.name == "nt":
            import msvcrt
            msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, 1)
        else:
            import fcntl
            fcntl.flock(handle.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        try:
            yield
        finally:
            if os.name == "nt":
                handle.seek(0)
                msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, 1)
            else:
                fcntl.flock(handle.fileno(), fcntl.LOCK_UN)


def disable_old(addons):
    """Turn off a former Aurora LiveMesh so Block's package can enable."""
    for name in (OLD_LEGACY, "bl_ext.user_default." + OLD_PKG):
        if name in addons:
            try:
                checked(bpy.ops.preferences.addon_disable(module=name))
            except Exception:
                pass


def install(request, state):
    extension, config, legacy, repo, target, module = state
    expected = request["plugin_version"]
    old = package_version(target, extension) or (package_version(legacy, False) if extension else None)
    if old and tuple(int(v) for v in old.split(".")) > tuple(int(v) for v in expected.split(".")):
        raise RuntimeError("A newer Block LiveMesh plug-in is already installed; downgrade refused.")
    package = Path(request["extension_zip"] if extension else request["legacy_zip"])
    with zipfile.ZipFile(package) as archive:
        files = {}
        for name in archive.namelist():
            relative = Path(name if extension else name.removeprefix(LEGACY_PKG + "/"))
            if relative.is_absolute() or ".." in relative.parts:
                raise RuntimeError("Invalid path in the bundled Block LiveMesh package.")
            if not name.endswith("/"):
                files[relative] = archive.read(name)
    if Path("__init__.py") not in files:
        raise RuntimeError("The bundled Block LiveMesh package is incomplete.")
    config.parent.mkdir(parents=True, exist_ok=True)
    config.mkdir(parents=True, exist_ok=True)
    backup = config.parent / "block-livemesh-backups" / (
        datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ-") + uuid.uuid4().hex
    )
    backup.mkdir(parents=True)
    prefs = config / "userpref.blend"
    before_addons = set(bpy.context.preferences.addons.keys())
    directories = [(target, backup / "package")]
    if extension and legacy != target:
        directories.append((legacy, backup / "legacy"))
    # Also snapshot a former Aurora extension folder so rollback stays clean.
    if extension and repo is not None:
        old_target = Path(repo.directory) / OLD_PKG
        if old_target.is_dir() and old_target != target:
            directories.append((old_target, backup / "aurora_package"))
    existed = []
    for path, saved in directories:
        if path.is_symlink() or (path.exists() and not path.is_dir()):
            raise RuntimeError("LiveMesh package location is not a regular directory: " + str(path))
        existed.append(path.is_dir())
        if path.is_dir():
            shutil.copytree(path, saved, symlinks=True)
    if prefs.is_file():
        shutil.copy2(prefs, backup / "userpref.blend")
    directories = [(path.resolve(), saved) for path, saved in directories]
    try:
        disable_old(before_addons)
        if extension and LEGACY_PKG in before_addons:
            checked(bpy.ops.preferences.addon_disable(module=LEGACY_PKG))
        if extension:
            checked(
                bpy.ops.extensions.package_install_files(
                    filepath=request["extension_zip"],
                    repo=repo.module,
                    enable_on_install=True,
                    overwrite=True,
                )
            )
        else:
            checked(bpy.ops.preferences.addon_install(filepath=request["legacy_zip"], overwrite=True))
            checked(bpy.ops.preferences.addon_enable(module=module))
        installed = importlib.import_module(module)
        if package_version(Path(installed.__file__).parent, extension) != expected:
            raise RuntimeError("Installed Block LiveMesh version does not match the bundled package.")
        for relative, contents in files.items():
            if (target / relative).read_bytes() != contents:
                raise RuntimeError(
                    "Blender did not install the complete bundled Block LiveMesh package: " + str(relative)
                )
        # Scene property name is still aurora_live inside the Python add-on.
        if not hasattr(bpy.types.Scene, "aurora_live") or module not in bpy.context.preferences.addons:
            raise RuntimeError("Block LiveMesh plug-in did not enable correctly.")
        after_addons = set(bpy.context.preferences.addons.keys())
        ignore = {module, LEGACY_PKG, OLD_LEGACY, "bl_ext.user_default." + OLD_PKG}
        if after_addons - ignore != before_addons - ignore:
            raise RuntimeError("Unrelated enabled add-ons changed; restoring the previous installation.")
        config.mkdir(parents=True, exist_ok=True)
        try:
            if AUTO_SAVE is not None:
                bpy.context.preferences.use_preferences_save = AUTO_SAVE
            checked(bpy.ops.wm.save_userpref())
        finally:
            if AUTO_SAVE is not None:
                bpy.context.preferences.use_preferences_save = False
        if not prefs.is_file():
            raise RuntimeError("Blender did not save preferences in the selected profile.")
    except Exception as error:
        try:
            for (path, saved), was_present in zip(directories, existed):
                if path.is_dir():
                    shutil.rmtree(path)
                if was_present:
                    shutil.copytree(saved, path, symlinks=True)
            config.mkdir(parents=True, exist_ok=True)
            if (backup / "userpref.blend").is_file():
                shutil.copy2(backup / "userpref.blend", prefs)
            elif prefs.is_file():
                prefs.unlink()
        except Exception as rollback_error:
            raise RuntimeError(str(error) + "; restore backup from " + str(backup) + ": " + str(rollback_error)) from error
        raise RuntimeError(str(error) + "; previous files restored; backup: " + str(backup)) from error
    return str(backup)


def uninstall(state):
    """Disable and delete Aurora's package from this profile, then save the
    preferences so Blender no longer lists it. Only the folders that hold
    Aurora's own package are deleted; the backups made by `install` stay,
    as copies of the user's own preferences."""
    extension, config, legacy, repo, target, module = state
    removed = []
    disabled = False
    for name in dict.fromkeys([module, "aurora_live"]):
        if name in bpy.context.preferences.addons:
            checked(bpy.ops.preferences.addon_disable(module=name))
            disabled = True
    for path, is_extension in dict.fromkeys([(target, extension), (legacy, False)]):
        if path.is_symlink() or not path.is_dir() or package_version(path, is_extension) is None:
            continue
        shutil.rmtree(path)
        removed.append(str(path))
    if disabled or removed:
        try:
            if AUTO_SAVE is not None:
                bpy.context.preferences.use_preferences_save = AUTO_SAVE
            checked(bpy.ops.wm.save_userpref())
        finally:
            if AUTO_SAVE is not None:
                bpy.context.preferences.use_preferences_save = False
    return removed


def run(request):
    report = {
        "version": list(bpy.app.version[:3]),
        "plugin_version": None,
        "enabled": False,
        "profile": "",
        "target": "",
        "backup": None,
        "error": None,
    }
    if bpy.app.version < (2, 93, 0):
        return report
    state = locations()
    extension, config, legacy, repo, target, module = state
    report["profile"], report["target"] = str(config.parent), str(target)
    report["plugin_version"] = package_version(target, extension) or (
        package_version(legacy, False) if extension else None
    )
    report["enabled"] = module in bpy.context.preferences.addons
    if request["operation"] == "install":
        with install_lock(config.parent / ".block-livemesh-install.lock"):
            report["backup"] = install(request, state)
            report["plugin_version"], report["enabled"] = request["plugin_version"], True
    elif request["operation"] == "uninstall":
        lock = config.parent / ".aurora-install.lock"
        with install_lock(lock):
            report["removed"] = uninstall(state)
            report["plugin_version"], report["enabled"] = None, False
        lock.unlink(missing_ok=True)
    return report


if __name__ == "__main__":
    arguments = sys.argv[sys.argv.index("--") + 1 :]
    request_file, report_file = map(Path, arguments)
    try:
        result = run(json.loads(request_file.read_text(encoding="utf-8")))
    except Exception as error:
        result = {"version": list(bpy.app.version[:3]), "error": str(error)}
    report_file.write_text(json.dumps(result, ensure_ascii=False), encoding="utf-8")
    if result.get("error"):
        raise RuntimeError(result["error"])
