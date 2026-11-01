//! Firestorm's catalogue of group abilities (indra/newview/skins/default/xui/
//! en/role_actions.xml, read by LLGroupMgr::parseRoleActions, originally
//! LGPL 2.1), generated: the sets in their order, each ability with its
//! power bit (roles_constants.h) and its texts (`tr!` keys).

/// One ability: its power bit, short and long texts.
pub struct Ability {
    pub bit: u32,
    pub name: &'static str,
    pub help: &'static str,
}

/// A set of abilities, its icon and its texts.
pub struct AbilitySet {
    pub icon: &'static str,
    pub name: &'static str,
    pub help: &'static str,
    pub abilities: Vec<Ability>,
}

pub fn catalogue() -> Vec<AbilitySet> {
    vec![
        AbilitySet {
            icon: "users",
            name: block_i18n::tr!("group-abilities-membership"),
            help: block_i18n::tr!("group-abilities-membership-help"),
            abilities: vec![
                Ability {
                    bit: 1,
                    name: block_i18n::tr!("group-ability-member-invite"),
                    help: block_i18n::tr!("group-ability-member-invite-help"),
                },
                Ability {
                    bit: 2,
                    name: block_i18n::tr!("group-ability-member-eject"),
                    help: block_i18n::tr!("group-ability-member-eject-help"),
                },
                Ability {
                    bit: 51,
                    name: block_i18n::tr!("group-ability-allow-ban"),
                    help: block_i18n::tr!("group-ability-allow-ban-help"),
                },
                Ability {
                    bit: 3,
                    name: block_i18n::tr!("group-ability-member-options"),
                    help: block_i18n::tr!("group-ability-member-options-help"),
                },
            ],
        },
        AbilitySet {
            icon: "crown",
            name: block_i18n::tr!("group-abilities-roles"),
            help: block_i18n::tr!("group-abilities-roles-help"),
            abilities: vec![
                Ability {
                    bit: 4,
                    name: block_i18n::tr!("group-ability-role-create"),
                    help: block_i18n::tr!("group-ability-role-create-help"),
                },
                Ability {
                    bit: 5,
                    name: block_i18n::tr!("group-ability-role-delete"),
                    help: block_i18n::tr!("group-ability-role-delete-help"),
                },
                Ability {
                    bit: 6,
                    name: block_i18n::tr!("group-ability-role-properties"),
                    help: block_i18n::tr!("group-ability-role-properties-help"),
                },
                Ability {
                    bit: 7,
                    name: block_i18n::tr!("group-ability-role-assign-member-limited"),
                    help: block_i18n::tr!("group-ability-role-assign-member-limited-help"),
                },
                Ability {
                    bit: 8,
                    name: block_i18n::tr!("group-ability-role-assign-member"),
                    help: block_i18n::tr!("group-ability-role-assign-member-help"),
                },
                Ability {
                    bit: 9,
                    name: block_i18n::tr!("group-ability-role-remove-member"),
                    help: block_i18n::tr!("group-ability-role-remove-member-help"),
                },
                Ability {
                    bit: 10,
                    name: block_i18n::tr!("group-ability-role-change-actions"),
                    help: block_i18n::tr!("group-ability-role-change-actions-help"),
                },
            ],
        },
        AbilitySet {
            icon: "id-filled",
            name: block_i18n::tr!("group-abilities-group-identity"),
            help: block_i18n::tr!("group-abilities-group-identity-help"),
            abilities: vec![Ability {
                bit: 11,
                name: block_i18n::tr!("group-ability-group-change-identity"),
                help: block_i18n::tr!("group-ability-group-change-identity-help"),
            }],
        },
        AbilitySet {
            icon: "map-2",
            name: block_i18n::tr!("group-abilities-parcel-management"),
            help: block_i18n::tr!("group-abilities-parcel-management-help"),
            abilities: vec![
                Ability {
                    bit: 12,
                    name: block_i18n::tr!("group-ability-land-deed"),
                    help: block_i18n::tr!("group-ability-land-deed-help"),
                },
                Ability {
                    bit: 13,
                    name: block_i18n::tr!("group-ability-land-release"),
                    help: block_i18n::tr!("group-ability-land-release-help"),
                },
                Ability {
                    bit: 14,
                    name: block_i18n::tr!("group-ability-land-set-sale-info"),
                    help: block_i18n::tr!("group-ability-land-set-sale-info-help"),
                },
                Ability {
                    bit: 15,
                    name: block_i18n::tr!("group-ability-land-divide-join"),
                    help: block_i18n::tr!("group-ability-land-divide-join-help"),
                },
            ],
        },
        AbilitySet {
            icon: "map-pin-filled",
            name: block_i18n::tr!("group-abilities-parcel-identity"),
            help: block_i18n::tr!("group-abilities-parcel-identity-help"),
            abilities: vec![
                Ability {
                    bit: 17,
                    name: block_i18n::tr!("group-ability-land-find-places"),
                    help: block_i18n::tr!("group-ability-land-find-places-help"),
                },
                Ability {
                    bit: 18,
                    name: block_i18n::tr!("group-ability-land-change-identity"),
                    help: block_i18n::tr!("group-ability-land-change-identity-help"),
                },
                Ability {
                    bit: 19,
                    name: block_i18n::tr!("group-ability-land-set-landing-point"),
                    help: block_i18n::tr!("group-ability-land-set-landing-point-help"),
                },
            ],
        },
        AbilitySet {
            icon: "adjustments-filled",
            name: block_i18n::tr!("group-abilities-parcel-settings"),
            help: block_i18n::tr!("group-abilities-parcel-settings-help"),
            abilities: vec![
                Ability {
                    bit: 20,
                    name: block_i18n::tr!("group-ability-land-change-media"),
                    help: block_i18n::tr!("group-ability-land-change-media-help"),
                },
                Ability {
                    bit: 21,
                    name: block_i18n::tr!("group-ability-land-edit"),
                    help: block_i18n::tr!("group-ability-land-edit-help"),
                },
                Ability {
                    bit: 22,
                    name: block_i18n::tr!("group-ability-land-options"),
                    help: block_i18n::tr!("group-ability-land-options-help"),
                },
                Ability {
                    bit: 46,
                    name: block_i18n::tr!("group-ability-land-change-environment"),
                    help: block_i18n::tr!("group-ability-land-change-environment-help"),
                },
            ],
        },
        AbilitySet {
            icon: "bolt-filled",
            name: block_i18n::tr!("group-abilities-parcel-powers"),
            help: block_i18n::tr!("group-abilities-parcel-powers-help"),
            abilities: vec![
                Ability {
                    bit: 23,
                    name: block_i18n::tr!("group-ability-land-allow-edit-land"),
                    help: block_i18n::tr!("group-ability-land-allow-edit-land-help"),
                },
                Ability {
                    bit: 24,
                    name: block_i18n::tr!("group-ability-land-allow-fly"),
                    help: block_i18n::tr!("group-ability-land-allow-fly-help"),
                },
                Ability {
                    bit: 25,
                    name: block_i18n::tr!("group-ability-land-allow-create"),
                    help: block_i18n::tr!("group-ability-land-allow-create-help"),
                },
                Ability {
                    bit: 26,
                    name: block_i18n::tr!("group-ability-land-allow-direct-teleport"),
                    help: block_i18n::tr!("group-ability-land-allow-direct-teleport-help"),
                },
                Ability {
                    bit: 28,
                    name: block_i18n::tr!("group-ability-land-allow-set-home"),
                    help: block_i18n::tr!("group-ability-land-allow-set-home-help"),
                },
                Ability {
                    bit: 41,
                    name: block_i18n::tr!("group-ability-land-allow-host-event"),
                    help: block_i18n::tr!("group-ability-land-allow-host-event-help"),
                },
            ],
        },
        AbilitySet {
            icon: "lock-filled",
            name: block_i18n::tr!("group-abilities-parcel-access"),
            help: block_i18n::tr!("group-abilities-parcel-access-help"),
            abilities: vec![
                Ability {
                    bit: 29,
                    name: block_i18n::tr!("group-ability-land-manage-allowed"),
                    help: block_i18n::tr!("group-ability-land-manage-allowed-help"),
                },
                Ability {
                    bit: 30,
                    name: block_i18n::tr!("group-ability-land-manage-banned"),
                    help: block_i18n::tr!("group-ability-land-manage-banned-help"),
                },
                Ability {
                    bit: 31,
                    name: block_i18n::tr!("group-ability-land-manage-passes"),
                    help: block_i18n::tr!("group-ability-land-manage-passes-help"),
                },
                Ability {
                    bit: 32,
                    name: block_i18n::tr!("group-ability-land-admin"),
                    help: block_i18n::tr!("group-ability-land-admin-help"),
                },
            ],
        },
        AbilitySet {
            icon: "photo-filled",
            name: block_i18n::tr!("group-abilities-parcel-content"),
            help: block_i18n::tr!("group-abilities-parcel-content-help"),
            abilities: vec![
                Ability {
                    bit: 48,
                    name: block_i18n::tr!("group-ability-land-return-group-owned"),
                    help: block_i18n::tr!("group-ability-land-return-group-owned-help"),
                },
                Ability {
                    bit: 33,
                    name: block_i18n::tr!("group-ability-land-return-group-set"),
                    help: block_i18n::tr!("group-ability-land-return-group-set-help"),
                },
                Ability {
                    bit: 34,
                    name: block_i18n::tr!("group-ability-land-return-non-group"),
                    help: block_i18n::tr!("group-ability-land-return-non-group-help"),
                },
                Ability {
                    bit: 35,
                    name: block_i18n::tr!("group-ability-land-gardening"),
                    help: block_i18n::tr!("group-ability-land-gardening-help"),
                },
            ],
        },
        AbilitySet {
            icon: "package",
            name: block_i18n::tr!("group-abilities-object-management"),
            help: block_i18n::tr!("group-abilities-object-management-help"),
            abilities: vec![
                Ability {
                    bit: 36,
                    name: block_i18n::tr!("group-ability-object-deed"),
                    help: block_i18n::tr!("group-ability-object-deed-help"),
                },
                Ability {
                    bit: 38,
                    name: block_i18n::tr!("group-ability-object-manipulate"),
                    help: block_i18n::tr!("group-ability-object-manipulate-help"),
                },
                Ability {
                    bit: 39,
                    name: block_i18n::tr!("group-ability-object-set-sale"),
                    help: block_i18n::tr!("group-ability-object-set-sale-help"),
                },
            ],
        },
        AbilitySet {
            icon: "coin-filled",
            name: block_i18n::tr!("group-abilities-accounting"),
            help: block_i18n::tr!("group-abilities-accounting-help"),
            abilities: vec![Ability {
                bit: 40,
                name: block_i18n::tr!("group-ability-accounting-accountable"),
                help: block_i18n::tr!("group-ability-accounting-accountable-help"),
            }],
        },
        AbilitySet {
            icon: "speakerphone",
            name: block_i18n::tr!("group-abilities-notices"),
            help: block_i18n::tr!("group-abilities-notices-help"),
            abilities: vec![
                Ability {
                    bit: 42,
                    name: block_i18n::tr!("group-ability-notices-send"),
                    help: block_i18n::tr!("group-ability-notices-send-help"),
                },
                Ability {
                    bit: 43,
                    name: block_i18n::tr!("group-ability-notices-receive"),
                    help: block_i18n::tr!("group-ability-notices-receive-help"),
                },
            ],
        },
        AbilitySet {
            icon: "messages-filled",
            name: block_i18n::tr!("group-abilities-chat"),
            help: block_i18n::tr!("group-abilities-chat-help"),
            abilities: vec![
                Ability {
                    bit: 16,
                    name: block_i18n::tr!("group-ability-join-group-chat"),
                    help: block_i18n::tr!("group-ability-join-group-chat-help"),
                },
                Ability {
                    bit: 27,
                    name: block_i18n::tr!("group-ability-join-voice-chat"),
                    help: block_i18n::tr!("group-ability-join-voice-chat-help"),
                },
                Ability {
                    bit: 37,
                    name: block_i18n::tr!("group-ability-moderate-group-chat"),
                    help: block_i18n::tr!("group-ability-moderate-group-chat-help"),
                },
            ],
        },
        AbilitySet {
            icon: "key-filled",
            name: block_i18n::tr!("group-abilities-experience-tools-experience"),
            help: block_i18n::tr!("group-abilities-experience-tools-experience-help"),
            abilities: vec![
                Ability {
                    bit: 49,
                    name: block_i18n::tr!("group-ability-experience-admin"),
                    help: block_i18n::tr!("group-ability-experience-admin-help"),
                },
                Ability {
                    bit: 50,
                    name: block_i18n::tr!("group-ability-experience-contributor"),
                    help: block_i18n::tr!("group-ability-experience-contributor-help"),
                },
            ],
        },
    ]
}
