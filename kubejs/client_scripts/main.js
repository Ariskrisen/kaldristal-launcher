JEIEvents.hideItems(event => {
    event.hide('iceandfire:armor_silver_metal_helmet')
    event.hide('iceandfire:armor_silver_metal_chestplate')
    event.hide('iceandfire:armor_silver_metal_leggings')
    event.hide('iceandfire:armor_silver_metal_boots')

    event.hide('iceandfire:silver_sword')
    event.hide('iceandfire:silver_shovel')
    event.hide('iceandfire:silver_pickaxe')
    event.hide('iceandfire:silver_axe')
    event.hide('iceandfire:silver_hoe')

    event.hide('eidolon:silver_helmet')
    event.hide('eidolon:silver_chestplate')
    event.hide('eidolon:silver_leggings')
    event.hide('eidolon:silver_boots')

    event.hide('eidolon:silver_sword')
    event.hide('eidolon:silver_shovel')
    event.hide('eidolon:silver_pickaxe')
    event.hide('eidolon:silver_axe')
    event.hide('eidolon:silver_hoe')
});

JEIEvents.hideItems(event => {
  [
    'sophisticatedbackpacks:magnet_upgrade',
    'sophisticatedbackpacks:deposit_upgrade',
    'sophisticatedbackpacks:tool_swapper_upgrade',
    'sophisticatedbackpacks:smelting_upgrade',
    'sophisticatedbackpacks:survival_infinity_upgrade',
    'sophisticatedbackpacks:advanced_feeding_upgrade',
    'sophisticatedbackpacks:advanced_filter_upgrade',
    'sophisticatedbackpacks:auto_blasting_upgrade',
    'sophisticatedbackpacks:restock_upgrade',
    'sophisticatedbackpacks:advanced_pump_upgrade',
    'sophisticatedbackpacks:battery_upgrade',
    'sophisticatedbackpacks:advanced_restock_upgrade',
    'sophisticatedbackpacks:auto_smelting_upgrade',
    'sophisticatedbackpacks:xp_pump_upgrade',
    'sophisticatedbackpacks:compacting_upgrade',
    'sophisticatedbackpacks:advanced_deposit_upgrade',
    'sophisticatedbackpacks:advanced_tool_swapper_upgrade',
    'sophisticatedbackpacks:blasting_upgrade',
    'sophisticatedbackpacks:stonecutter_upgrade',
    'sophisticatedbackpacks:stack_upgrade_starter_tier',
    'sophisticatedbackpacks:stack_upgrade_tier_1',
	'sophisticatedbackpacks:stack_upgrade_tier_2',
	'sophisticatedbackpacks:stack_upgrade_tier_3',
	'sophisticatedbackpacks:stack_downgrade_tier_1',
	'sophisticatedbackpacks:stack_downgrade_tier_2',
	'sophisticatedbackpacks:stack_downgrade_tier_3',
    'sophisticatedbackpacks:stack_upgrade_tier_4',
    'sophisticatedbackpacks:stack_upgrade_omega_tier',
    'sophisticatedbackpacks:anvil_upgrade',
    'sophisticatedbackpacks:inception_upgrade',
    'sophisticatedbackpacks:feeding_upgrade',
    'sophisticatedbackpacks:auto_smoking_upgrade',
    'sophisticatedbackpacks:advanced_compacting_upgrade',
    'sophisticatedbackpacks:infinity_upgrade',
    'sophisticatedbackpacks:pump_upgrade',
    'sophisticatedbackpacks:advanced_alchemy_upgrade',
    'sophisticatedbackpacks:filter_upgrade',
    'sophisticatedbackpacks:alchemy_upgrade',
    'sophisticatedbackpacks:everlasting_upgrade',
    'sophisticatedbackpacks:smoking_upgrade',
    'sophisticatedbackpacks:advanced_magnet_upgrade',
    'sophisticatedbackpacks:smithing_upgrade',
    'sophisticatedcore:xp_bucket',
  ].forEach(item => {
    event.hide(item)
  })
})

JEIEvents.hideFluids(event => {
    event.hide('sophisticatedcore:xp_still')
})
