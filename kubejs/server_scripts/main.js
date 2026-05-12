ServerEvents.highPriorityData(event => {
  let swords = [
    "weapon_fates_beacon",
    "weapon_kingslayer",
    "weapon_volcano_flame",
    "weapon_phantom_needle",
    "weapon_desert_wind",
    "weapon_estoc",
    "weapon_hefty_club"
  ]

  let daggers = [
    "weapon_misericorde",
    "weapon_wakizashi",
    "weapon_fog_guard_dagger"
  ]

  let katanas = [
    "weapon_nagakiba",
    "weapon_hand_of_malenia",
    "weapon_rivers_of_blood"
  ]

    let spears = [
    "weapon_spark_of_dawn_polearm",
    "weapon_chess_board_knight_polearm",
    "weapon_fury_of_the_storm"
  ]

  let scythes = [
    "weapon_crimson_reaper",
    "weapon_dragons_doom"
  ]

  let cleavers = [
    "flint_cleaver",
    "iron_cleaver",
    "golden_cleaver",
    "diamond_cleaver",
    "netherite_cleaver",
    "stained_cleaver",
    "steeleaf_cleaver",
    "knightmetal_cleaver",
    "fiery_cleaver"
  ]

  cleavers.forEach(cleaver => {
    event.addJson(`dungeonsdelight:weapon_attributes/${cleaver}.json`, {
      parent: "bettercombat:axe"
    })
  })

  event.addJson("dungeonsdelight:weapon_attributes/ironwood_cleaver.json", {
    parent: "bettercombat:dagger"
  })

  swords.forEach(sword => {
    event.addJson(`fantasy_weapons:weapon_attributes/${sword}.json`, {
      parent: "bettercombat:sword"
    })
  })

  daggers.forEach(dagger => {
    event.addJson(`fantasy_weapons:weapon_attributes/${dagger}.json`, {
      parent: "bettercombat:dagger"
    })
  })

  katanas.forEach(katana => {
    event.addJson(`fantasy_weapons:weapon_attributes/${katana}.json`, {
      parent: "bettercombat:katana"
    })
  })

  spears.forEach(spear => {
    event.addJson(`fantasy_weapons:weapon_attributes/${spear}.json`, {
      parent: "bettercombat:spear"
    })
  })

  scythes.forEach(scythe => {
    event.addJson(`fantasy_weapons:weapon_attributes/${scythe}.json`, {
      parent: "bettercombat:scythe"
    })
  })

  event.addJson("fantasy_weapons:weapon_attributes/weapon_spider_fang.json", {
    parent: "bettercombat:heavy_axe"
  })

  event.addJson("cataclysm:weapon_attributes/the_immolator.json", {
    parent: "bettercombat:mace"
  })

  event.addJson("fantasy_weapons:weapon_attributes/weapon_zweihander.json", {
    parent: "bettercombat:claymore"
  })

  event.addJson("fantasy_weapons:weapon_attributes/weapon_zweihander.json", {
    parent: "bettercombat:claymore"
  })


})

ServerEvents.recipes(event => {

    event.remove({ output: 'iceandfire:armor_silver_metal_helmet' })
    event.remove({ output: 'iceandfire:armor_silver_metal_chestplate' })
    event.remove({ output: 'iceandfire:armor_silver_metal_leggings' })
    event.remove({ output: 'iceandfire:armor_silver_metal_boots' })

    event.remove({ output: 'iceandfire:silver_sword' })
    event.remove({ output: 'iceandfire:silver_showel' })
    event.remove({ output: 'iceandfire:silver_pickaxe' })
    event.remove({ output: 'iceandfire:silver_axe' })
    event.remove({ output: 'iceandfire:silver_hoe' })

    event.remove({ output: 'eidolon:silver_helmet' })
    event.remove({ output: 'eidolon:silver_chestplate' })
    event.remove({ output: 'eidolon:silver_leggings' })
    event.remove({ output: 'eidolon:silver_boots' })

    event.remove({ output: 'eidolon:silver_sword' })
    event.remove({ output: 'eidolon:silver_showel' })
    event.remove({ output: 'eidolon:silver_pickaxe' })
    event.remove({ output: 'eidolon:silver_axe' })
    event.remove({ output: 'eidolon:silver_hoe' })
	
	event.remove({ output: 'twilightforest:arctic_fur_block' })


    event.remove({ output: 'paladins:golden_claymore' })

event.shaped(
  Item.of('paladins:golden_claymore', 1), 
  [
    '  A',
    'AA ', 
    'BA '
  ],
  {
    A: 'minecraft:gold_ingot',
    B: 'minecraft:stick'
  }
)

    event.remove({ output: 'paladins:diamond_claymore' })

event.shaped(
  Item.of('paladins:diamond_claymore', 1), 
  [
    '  A',
    'AA ', 
    'BA '
  ],
  {
    A: 'minecraft:diamond',
    B: 'minecraft:stick'
  }
)

    event.remove({ output: 'paladins:iron_claymore' })

event.shaped(
  Item.of('paladins:iron_claymore', 1), 
  [
    '  A',
    'AA ', 
    'BA '
  ],
  {
    A: 'minecraft:iron_ingot',
    B: 'minecraft:stick'
  }
)

    event.remove({ output: 'paladins:stone_claymore' })

event.shaped(
  Item.of('paladins:stone_claymore', 1), 
  [
    '  A',
    'AA ', 
    'BA '
  ],
  {
    A: 'minecraft:cobblestone',
    B: 'minecraft:stick'
  }
)

    event.remove({ output: 'paladins:iron_mace' })

event.shaped(
  Item.of('paladins:iron_mace', 1), 
  [
    ' A ',
    'BA ', 
    '   '
  ],
  {
    A: 'minecraft:iron_ingot',
    B: 'minecraft:stick'
  }
)

    event.remove({ output: 'paladins:golden_mace' })

event.shaped(
  Item.of('paladins:golden_mace', 1), 
  [
    ' A ',
    'BA ', 
    '   '
  ],
  {
    A: 'minecraft:gold_ingot',
    B: 'minecraft:stick'
  }
)

    event.remove({ output: 'paladins:diamond_mace' })

event.shaped(
  Item.of('paladins:diamond_mace', 1), 
  [
    ' A ',
    'BA ', 
    '   '
  ],
  {
    A: 'minecraft:diamond',
    B: 'minecraft:stick'
  }
)


    event.remove({ output: 'rogues:diamond_glaive' })

event.shaped(
  Item.of('rogues:diamond_glaive', 1), 
  [
    ' AA',
    'AB ', 
    'B  '
  ],
  {
    A: 'minecraft:diamond',
    B: 'minecraft:stick'
  }
)

    event.remove({ output: 'rogues:iron_glaive' })

event.shaped(
  Item.of('rogues:iron_glaive', 1), 
  [
    ' AA',
    'AB ', 
    'B  '
  ],
  {
    A: 'minecraft:iron_ingot',
    B: 'minecraft:stick'
  }
)

    event.remove({ output: 'rogues:golden_glaive' })

event.shaped(
  Item.of('rogues:golden_glaive', 1), 
  [
    ' AA',
    'AB ', 
    'B  '
  ],
  {
    A: 'minecraft:gold_ingot',
    B: 'minecraft:stick'
  }
)

    event.remove({ output: 'archers:rapid_crossbow' })

event.shaped(
  Item.of('archers:rapid_crossbow', 1), 
  [
    'BFB',
    'CDC', 
    ' A '
  ],
  {
    A: 'minecraft:iron_ingot',
    B: 'minecraft:stick',
    C: 'minecraft:string',
    D: 'minecraft:tripwire_hook',
    F: 'minecraft:redstone'
  }
)

    event.remove({ output: 'archers:royal_longbow' })

event.shaped(
  Item.of('archers:royal_longbow', 1), 
  [
    'DBC',
    'F C', 
    'DBC'
  ],
  {
    B: 'minecraft:stick',
    C: 'minecraft:string',
    D: 'minecraft:diamond',
    F: 'minecraft:gold_ingot'
  }
)

    event.remove({ output: 'archers:composite_longbow' })

event.shaped(
  Item.of('archers:composite_longbow', 1), 
  [
    ' BC',
    'A C', 
    ' BC'
  ],
  {
	A: 'minecraft:bone',  
    B: 'minecraft:stick',
    C: 'minecraft:string'
  }
)

 event.remove({ output: 'rogues:iron_double_axe' })

event.shaped(
  Item.of('rogues:iron_double_axe', 1), 
  [
    'ABA',
    'ABA', 
    ' B '
  ],
  {
    A: 'minecraft:iron_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:golden_double_axe' })

event.shaped(
  Item.of('rogues:golden_double_axe', 1), 
  [
    'ABA',
    'ABA', 
    ' B '
  ],
  {
    A: 'minecraft:gold_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:diamond_double_axe' })

event.shaped(
  Item.of('rogues:diamond_double_axe', 1), 
  [
    'ABA',
    'ABA', 
    ' B '
  ],
  {
    A: 'minecraft:diamond',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:stone_double_axe' })

event.shaped(
  Item.of('rogues:stone_double_axe', 1), 
  [
    'ABA',
    'ABA', 
    ' B '
  ],
  {
    A: 'minecraft:cobblestone',
    B: 'minecraft:stick'
  }
)


 event.remove({ output: 'paladins:diamond_great_hammer' })

event.shaped(
  Item.of('paladins:diamond_great_hammer', 1), 
  [
    ' AA',
    ' AA', 
    'B  '
  ],
  {
    A: 'minecraft:diamond',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'paladins:stone_great_hammer' })

event.shaped(
  Item.of('paladins:stone_great_hammer', 1), 
  [
    ' AA',
    ' AA', 
    'B  '
  ],
  {
    A: 'minecraft:cobblestone',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'paladins:iron_great_hammer' })

event.shaped(
  Item.of('paladins:iron_great_hammer', 1), 
  [
    ' AA',
    ' AA', 
    'B  '
  ],
  {
    A: 'minecraft:iron_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'paladins:golden_great_hammer' })

event.shaped(
  Item.of('paladins:golden_great_hammer', 1), 
  [
    ' AA',
    ' AA', 
    'B  '
  ],
  {
    A: 'minecraft:gold_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'paladins:wooden_great_hammer' })

event.shaped(
  Item.of('paladins:wooden_great_hammer', 1), 
  [
    ' AA',
    ' AA', 
    'B  '
  ],
  {
    A: '#minecraft:logs',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'archers:diamond_spear' })

event.shaped(
  Item.of('archers:diamond_spear', 1), 
  [
    '  A',
    ' B ', 
    'B  '
  ],
  {
    A: 'minecraft:diamond',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'archers:iron_spear' })

event.shaped(
  Item.of('archers:iron_spear', 1), 
  [
    '  A',
    ' B ', 
    'B  '
  ],
  {
    A: 'minecraft:iron_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'archers:golden_spear' })

event.shaped(
  Item.of('archers:golden_spear', 1), 
  [
    '  A',
    ' B ', 
    'B  '
  ],
  {
    A: 'minecraft:gold_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'archers:flint_spear' })

event.shaped(
  Item.of('archers:flint_spear', 1), 
  [
    '  A',
    ' B ', 
    'B  '
  ],
  {
    A: 'minecraft:flint',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:diamond_sickle' })

event.shaped(
  Item.of('rogues:diamond_sickle', 1), 
  [
    'AA ',
    'B  ', 
    '   '
  ],
  {
    A: 'minecraft:diamond',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:iron_sickle' })

event.shaped(
  Item.of('rogues:iron_sickle', 1), 
  [
    'AA ',
    'B  ', 
    '   '
  ],
  {
    A: 'minecraft:iron_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:golden_sickle' })

event.shaped(
  Item.of('rogues:golden_sickle', 1), 
  [
    'AA ',
    'B  ', 
    '   '
  ],
  {
    A: 'minecraft:gold_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:iron_dagger' })

event.shaped(
  Item.of('rogues:iron_dagger', 1), 
  [
    ' A ',
    'B  ', 
    '   '
  ],
  {
    A: 'minecraft:iron_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:golden_dagger' })

event.shaped(
  Item.of('rogues:golden_dagger', 1), 
  [
    ' A ',
    'B  ', 
    '   '
  ],
  {
    A: 'minecraft:gold_ingot',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:flint_dagger' })

event.shaped(
  Item.of('rogues:flint_dagger', 1), 
  [
    ' A ',
    'B  ', 
    '   '
  ],
  {
    A: 'minecraft:flint',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'rogues:diamond_dagger' })

event.shaped(
  Item.of('rogues:diamond_dagger', 1), 
  [
    ' A ',
    'B  ', 
    '   '
  ],
  {
    A: 'minecraft:diamond',
    B: 'minecraft:stick'
  }
)

 event.remove({ output: 'arts_and_crafts_compat:bleached_knitted_wool' })

event.shaped(
  Item.of('arts_and_crafts_compat:bleached_knitted_wool', 1), 
  [
    'AA ',
    'AA ', 
    '   '
  ],
  {
    A: 'arts_and_crafts:bleached_wool'
  }
)

 event.remove({ output: 'arts_and_crafts_compat:bleached_knitted_carpet' })

event.shaped(
  Item.of('arts_and_crafts_compat:bleached_knitted_carpet', 1), 
  [
    '   ',
    'AA ', 
    '   '
  ],
  {
    A: 'arts_and_crafts_compat:bleached_knitted_wool'
  }
)

 event.remove({ output: 'backpacked:backpack' })

event.shaped(
  Item.of('backpacked:backpack', 1), 
  [
    'BAB',
    'CDC', 
    'BDB'
  ],
  {
    A: 'minecraft:rabbit_hide',
    B: 'minecraft:leather',
    C: 'minecraft:gold_ingot',
    D: '#minecraft:wool',
  }
)

event.shaped(
  Item.of('skilltree:amnesia_scroll', 1), 
  [
    'BBB',
    'BAB', 
    'BBB'
  ],
  {
    A: 'minecraft:paper',
    B: 'irons_spellbooks:ancient_knowledge_fragment'
  }
)
  let items = [
        'endrem:black_eye',
    'endrem:cold_eye',
    'endrem:corrupted_eye',
    'endrem:lost_eye',
    'endrem:nether_eye',
    'endrem:old_eye',
    'endrem:rogue_eye',
    'endrem:cursed_eye',
    'endrem:evil_eye',
    'endrem:guardian_eye',
    'endrem:magical_eye',
    'endrem:wither_eye',
    'endrem:witch_eye',
    'endrem:undead_eye',
    'endrem:exotic_eye',
    'endrem:cryptic_eye'
  ]

  event.shapeless(
    Item.of('origins:orb_of_origin'),
    Array(9).fill(items)
  )

  event.shapeless(
    Item.of('antiqueatlas:empty_antique_atlas', 1), 
  [
    'minecraft:compass',
    'minecraft:book' 	      
  ]
  )
});



