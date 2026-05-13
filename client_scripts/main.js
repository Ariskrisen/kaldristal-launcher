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

JEIEvents.hideItems(event => {
    // Список всех ID из лога
    const ids = [
        'backpacked:backpack',
        'backpacked:acacia_backpack_shelf',
        'backpacked:birch_backpack_shelf',
        'backpacked:cherry_backpack_shelf',
        'backpacked:crimson_backpack_shelf',
        'backpacked:dark_oak_backpack_shelf',
        'backpacked:jungle_backpack_shelf',
        'backpacked:oak_backpack_shelf',
        'backpacked:spruce_backpack_shelf',
        'backpacked:warped_backpack_shelf',
        'everycomp:bp/alexscaves/pewen_backpack_shelf',
        'everycomp:bp/alexscaves/thornwood_backpack_shelf',
        'everycomp:bp/arts_and_crafts/cork_backpack_shelf',
        'everycomp:bp/atmospheric/aspen_backpack_shelf',
        'everycomp:bp/atmospheric/grimwood_backpack_shelf',
        'everycomp:bp/atmospheric/kousa_backpack_shelf',
        'everycomp:bp/atmospheric/laurel_backpack_shelf',
        'everycomp:bp/atmospheric/morado_backpack_shelf',
        'everycomp:bp/atmospheric/rosewood_backpack_shelf',
        'everycomp:bp/atmospheric/yucca_backpack_shelf',
        'everycomp:bp/autumnity/maple_backpack_shelf',
        'everycomp:bp/biomesoplenty/dead_backpack_shelf',
        'everycomp:bp/biomesoplenty/fir_backpack_shelf',
        'everycomp:bp/biomesoplenty/hellbark_backpack_shelf',
        'everycomp:bp/biomesoplenty/jacaranda_backpack_shelf',
        'everycomp:bp/biomesoplenty/magic_backpack_shelf',
        'everycomp:bp/biomesoplenty/mahogany_backpack_shelf',
        'everycomp:bp/biomesoplenty/palm_backpack_shelf',
        'everycomp:bp/biomesoplenty/redwood_backpack_shelf',
        'everycomp:bp/biomesoplenty/umbran_backpack_shelf',
        'everycomp:bp/biomesoplenty/willow_backpack_shelf',
        'everycomp:bp/caverns_and_chasms/azalea_backpack_shelf',
        'everycomp:bp/ecologics/azalea_backpack_shelf',
        'everycomp:bp/ecologics/coconut_backpack_shelf',
        'everycomp:bp/ecologics/walnut_backpack_shelf',
        'everycomp:bp/eidolon/illwood_backpack_shelf',
        'everycomp:bp/endergetic/poise_backpack_shelf',
        'everycomp:bp/environmental/pine_backpack_shelf',
        'everycomp:bp/environmental/plum_backpack_shelf',
        'everycomp:bp/environmental/willow_backpack_shelf',
        'everycomp:bp/environmental/wisteria_backpack_shelf',
        'everycomp:bp/gardens_of_the_dead/soulblight_backpack_shelf',
        'everycomp:bp/minecells/putrid_backpack_shelf',
        'everycomp:bp/netherexp/claret_backpack_shelf',
        'everycomp:bp/quark/ancient_backpack_shelf',
        'everycomp:bp/quark/azalea_backpack_shelf',
        'everycomp:bp/quark/blossom_backpack_shelf',
        'everycomp:bp/twilightforest/canopy_backpack_shelf',
        'everycomp:bp/twilightforest/dark_backpack_shelf',
        'everycomp:bp/twilightforest/mangrove_backpack_shelf',
        'everycomp:bp/twilightforest/mining_backpack_shelf',
        'everycomp:bp/twilightforest/sorting_backpack_shelf',
        'everycomp:bp/twilightforest/time_backpack_shelf',
        'everycomp:bp/twilightforest/transformation_backpack_shelf',
        'everycomp:bp/twilightforest/twilight_oak_backpack_shelf',
        'everycomp:bp/upgrade_aquatic/driftwood_backpack_shelf',
        'everycomp:bp/upgrade_aquatic/river_backpack_shelf',
        'everycomp:bp/vinery/dark_cherry_backpack_shelf'
    ]
    
    ids.forEach(id => {
        const stack = Item.of(id)
        if (!stack.isEmpty()) {
            event.hide(stack)
        }
    })
    
    // Зачарованные книги с зачарованиями backpacked
    const enchants = [
        'backpacked:funnelling',
        'backpacked:imbued_hide',
        'backpacked:looted',
        'backpacked:marksman',
        'backpacked:repairman'
    ]
    
    enchants.forEach(id => {
        const book = Item.of('minecraft:enchanted_book').enchant(id, 1)
        if (!book.isEmpty()) {
            event.hide(book)
        }
    })
})
