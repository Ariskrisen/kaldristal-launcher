ItemEvents.modification(event => {
  event.modify('block_factorys_bosses:large_sword', item => {
    item.attackSpeed  = -3.5  
  })

  event.modify('minecells:broadsword', item => {
    item.attackSpeed  = -3.5  
  })
})


WorldgenEvents.remove(event => {
    event.removeOres(ores => {
        ores.blocks = [
            'eidolon:silver_ore',
            'iceandfire:silver_ore',
            'oreganized:silver_ore',
            'eidolon:deep_silver_ore',
            'iceandfire:deepslate_silver_ore',
            'oreganized:deepslate_silver_ore'
        ];
    });
});

StartupEvents.modifyCreativeTab('eidolon:general', event => {
    
    event.removeDisplay('eidolon:silver_helmet')
    event.removeDisplay('eidolon:silver_chestplate')
    event.removeDisplay('eidolon:silver_leggings')
    event.removeDisplay('eidolon:silver_boots')

    event.removeDisplay('eidolon:silver_sword')
    event.removeDisplay('eidolon:silver_showel')
    event.removeDisplay('eidolon:silver_pickaxe')
    event.removeDisplay('eidolon:silver_axe')
    event.removeDisplay('eidolon:silver_hoe')
});

StartupEvents.modifyCreativeTab('iceandfire:items', event => {
    event.removeDisplay('iceandfire:armor_silver_metal_helmet')
    event.removeDisplay('iceandfire:armor_silver_metal_chestplate')
    event.removeDisplay('iceandfire:armor_silver_metal_leggings')
    event.removeDisplay('iceandfire:armor_silver_metal_boots')

    event.removeDisplay('iceandfire:silver_sword')
    event.removeDisplay('iceandfire:silver_showel')
    event.removeDisplay('iceandfire:silver_pickaxe')
    event.removeDisplay('iceandfire:silver_axe')
    event.removeDisplay('iceandfire:silver_hoe')
});