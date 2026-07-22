extends TileMapLayer

var tileset_sources: Dictionary = {}
var loaded_chunks: Dictionary = {}

func _ready() -> void:
	_setup_tileset()

func _setup_tileset() -> void:
	var ts = TileSet.new()
	ts.tile_size = Vector2i(Constants.TILE_SIZE_PX, Constants.TILE_SIZE_PX)

	var floor_tex = load("res://sprites/Environment/Tilesets/Floors_Tiles.png")
	var wall_tex = load("res://sprites/Environment/Tilesets/Wall_Tiles.png")
	var water_tex = load("res://sprites/Environment/Tilesets/Water_tiles.png")

	var floor_source = TileSetAtlasSource.new()
	floor_source.texture = floor_tex
	floor_source.texture_region_size = Vector2i(Constants.TILE_SIZE_PX, Constants.TILE_SIZE_PX)
	_add_tiles_to_source(floor_source, 25, 26)
	ts.add_source(floor_source, 1)
	tileset_sources["floors"] = 1

	var wall_source = TileSetAtlasSource.new()
	wall_source.texture = wall_tex
	wall_source.texture_region_size = Vector2i(Constants.TILE_SIZE_PX, Constants.TILE_SIZE_PX)
	_add_tiles_to_source(wall_source, 25, 25)
	ts.add_source(wall_source, 2)
	tileset_sources["walls"] = 2

	var water_source = TileSetAtlasSource.new()
	water_source.texture = water_tex
	water_source.texture_region_size = Vector2i(Constants.TILE_SIZE_PX, Constants.TILE_SIZE_PX)
	_add_tiles_to_source(water_source, 25, 25)
	ts.add_source(water_source, 3)
	tileset_sources["water"] = 3

	tile_set = ts

func _add_tiles_to_source(source: TileSetAtlasSource, cols: int, rows: int) -> void:
	for y in range(rows):
		for x in range(cols):
			source.create_tile(Vector2i(x, y))

func _chunk_to_global(chunk_x: int, chunk_y: int, tx: int, ty: int) -> Vector2i:
	return Vector2i(chunk_x * Constants.CHUNK_SIZE + tx, chunk_y * Constants.CHUNK_SIZE + ty)

func _clear_chunk_tiles(chunk_x: int, chunk_y: int) -> void:
	var chunk_key = Vector2i(chunk_x, chunk_y)
	if loaded_chunks.has(chunk_key):
		var chunk_data = loaded_chunks[chunk_key]
		var tiles_arr: Array = chunk_data.tiles
		for ty in range(tiles_arr.size()):
			for tx in range(tiles_arr[ty].size()):
				var gc = _chunk_to_global(chunk_x, chunk_y, tx, ty)
				erase_cell(gc)
		loaded_chunks.erase(chunk_key)

func apply_chunk(data: Dictionary) -> void:
	var chunk_x: int = data.chunk_x
	var chunk_y: int = data.chunk_y
	var tiles: Array = data.tiles

	_clear_chunk_tiles(chunk_x, chunk_y)

	for ty in range(tiles.size()):
		var row: Array = tiles[ty]
		for tx in range(row.size()):
			var tile_id: int = row[tx]
			_paint_tile(chunk_x, chunk_y, tx, ty, tile_id)

	loaded_chunks[Vector2i(chunk_x, chunk_y)] = data

func update_tile(data: Dictionary) -> void:
	var chunk_x: int = data.chunk_x
	var chunk_y: int = data.chunk_y
	var tile_x: int = data.tile_x
	var tile_y: int = data.tile_y

	var chunk_key = Vector2i(chunk_x, chunk_y)
	if loaded_chunks.has(chunk_key):
		var chunk_data = loaded_chunks[chunk_key]
		var tiles_arr: Array = chunk_data.tiles
		if tile_y < tiles_arr.size() and tile_x < tiles_arr[tile_y].size():
			var tile_id: int = tiles_arr[tile_y][tile_x]
			_paint_tile(chunk_x, chunk_y, tile_x, tile_y, tile_id)

func _paint_tile(chunk_x: int, chunk_y: int, tx: int, ty: int, tile_id: int) -> void:
	var atlas_info = Constants.TILE_ATLAS_MAP.get(tile_id)
	if not atlas_info:
		atlas_info = Constants.TILE_ATLAS_MAP[0]

	var source_id = tileset_sources.get(atlas_info.source, 1)
	var global_coords = _chunk_to_global(chunk_x, chunk_y, tx, ty)
	set_cell(global_coords, source_id, atlas_info.atlas, 0)
