# rungeon
A Dungeons &amp; Dragons 5th Edition dungeon generator written in Rust.

## Checklist
- Tiles: COMPLETE
- Grids: COMPLETE
- Rooms: IN PROGRESS
  - Starting Areas: IN PROGRESS (4/10)
    - 1: COMPLETE
    - 2: COMPLETE
    - 3: COMPLETE
    - 4: COMPLETE
    - 5: IN PROGRESS
  - Passages: ON HOLD
  - Chambers: ON HOLD
- Maps: ON HOLD

## Considerations
- It might be wise to skip ahead to maps before getting too far along in creating rooms (even though it's much easier now). I'm still not sure if I'll need exit objects or modified room objects to specify the exit sizes when procedurally generating the adjoining rooms in a map. I could probably make do with starting_area_1 (all passages), starting_area_3 (all doors), and at least one passage room.

## Old To-Do list
1. Established passage width in starting_area needs to be passed out to the map so correctly sized passages can be generated to extend from them (exit object? can other exit types have widths?)
2. Passages need to be built out so map generation can continue
3. Map generation will continue until we eventually learn how chambers will be implemented
