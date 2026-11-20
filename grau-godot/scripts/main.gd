extends Node3D

# Cidade procedimental — Grau Urbano (G02-F2/G02-F3)
# Planta: dois quarteirões ligados por ruas com cruzamento X

const BLOCK_SIZE = 60.0      # lado do quarteirão (m)
const STREET_WIDTH = 12.0    # largura da rua (m)

@onready var route_manager = RouteManager.new()

func _ready() -> void:
	generate_city()
	_create_player()
	_setup_waypoints()
	add_child(route_manager)
	_setup_hud()
	print("[Grau Urbano] Cidade gerada: 2 quarteirões, cruzamento X, SP/PR1/PR2 marcados")
	print("[Grau Urbano] Player instanciado em SP (-132, 1, 132)")
	print("[Grau Urbano] Waypoints PN/PG/BC/C1 registados. Pressione '1' para Rota A, '2' para Rota B.")

func generate_city() -> void:
	# Chão da cidade (grama entre ruas)
	var ground = MeshInstance3D.new()
	ground.mesh = PlaneMesh.new()
	ground.mesh.size = Vector2(BLOCK_SIZE * 2 + STREET_WIDTH * 3, BLOCK_SIZE * 2 + STREET_WIDTH * 3)
	ground.material = _material(Color(0.15, 0.35, 0.15))
	add_child(ground)

	# Ruas (planos XZ, cor asfalto)
	_create_street(Vector3(-BLOCK_SIZE - STREET_WIDTH/2, 0.01, 0.0), Vector2(STREET_WIDTH, BLOCK_SIZE * 2 + STREET_WIDTH))  # oeste (N-S)
	_create_street(Vector3(0.0, 0.01, 0.0), Vector2(STREET_WIDTH, BLOCK_SIZE * 2 + STREET_WIDTH))                        # central (N-S)
	_create_street(Vector3(BLOCK_SIZE + STREET_WIDTH/2, 0.01, 0.0), Vector2(STREET_WIDTH, BLOCK_SIZE * 2 + STREET_WIDTH)) # leste (N-S)
	_create_street(Vector3(0.0, 0.01, -BLOCK_SIZE - STREET_WIDTH/2), Vector2(BLOCK_SIZE * 2 + STREET_WIDTH, STREET_WIDTH)) # norte (E-O)
	_create_street(Vector3(0.0, 0.01, 0.0), Vector2(BLOCK_SIZE * 2 + STREET_WIDTH, STREET_WIDTH))                        # central (E-O)
	_create_street(Vector3(0.0, 0.01, BLOCK_SIZE + STREET_WIDTH/2), Vector2(BLOCK_SIZE * 2 + STREET_WIDTH, STREET_WIDTH)) # sul (E-O)

	# Predios (8 total, 4 por quarteirão)
	_building(Vector3(-90, 10, 90), Vector3(20, 30, 20), Color(0.4, 0.35, 0.3))   # QTA NW
	_building(Vector3(-45, 15, 45), Vector3(15, 40, 15), Color(0.35, 0.3, 0.25))  # QTA NW
	_building(Vector3(-90, 8, 30), Vector3(20, 20, 20), Color(0.45, 0.4, 0.35))   # QTA NW
	_building(Vector3(-30, 12, 90), Vector3(12, 30, 12), Color(0.3, 0.3, 0.35))   # QTA NW
	_building(Vector3(90, 10, 90), Vector3(20, 30, 20), Color(0.4, 0.35, 0.3))    # QTb NE
	_building(Vector3(45, 15, 45), Vector3(15, 40, 15), Color(0.35, 0.3, 0.25))   # QTb NE
	_building(Vector3(90, 8, 30), Vector3(20, 20, 20), Color(0.45, 0.4, 0.35))    # QTb NE
	_building(Vector3(30, 12, 90), Vector3(12, 30, 12), Color(0.3, 0.3, 0.35))    # QTb NE

	# Marcadores originais
	_marker("SP", Vector3(-BLOCK_SIZE - STREET_WIDTH, 0.5, BLOCK_SIZE + STREET_WIDTH), Color(0, 0, 1))   # inferior esquerdo
	_marker("PR1", Vector3(BLOCK_SIZE + STREET_WIDTH, 0.5, -BLOCK_SIZE - STREET_WIDTH), Color(0, 1, 0)) # superior direito
	_marker("PR2", Vector3(BLOCK_SIZE + STREET_WIDTH, 0.5, BLOCK_SIZE + STREET_WIDTH), Color(1, 0, 0))   # inferior direito
	_marker("X", Vector3(0, 0.5, 0), Color(1, 1, 0))                                                    # cruzamento

	print("[Grau Urbano] SP=(-132, 0.5, 132), PR1=(132, 0.5, -132), PR2=(132, 0.5, 132), X=(0,0.5,0)")

func _setup_waypoints() -> void:
	# Waypoints de rota (G02-F3) - adiciona grupo "waypoints" para RouteManager encontrar
	# PG (Posto/Garagem) - spawn point, início da Rota A e fim da Rota B
	var pg = _waypoint("PG", Vector3(0, 0.5, -BLOCK_SIZE - STREET_WIDTH), Color(0.5, 1.0, 0.5), 1, 0)  # Rota A, índice 0
	pg.add_to_group("waypoints")
	
	# C1 (Cruzamento Central) - nó central, cruzado 3x no round trip
	var c1_a = _waypoint("C1", Vector3(0, 0.5, 0), Color(1.0, 1.0, 0.0), 1, 1)  # Rota A, índice 1
	c1_a.add_to_group("waypoints")
	
	# PN (Praça Norte) - turnaround da Rota A, início da Rota B
	var pn = _waypoint("PN", Vector3(0, 0.5, BLOCK_SIZE + STREET_WIDTH), Color(0.0, 0.5, 1.0), 1, 2)  # Rota A, índice 2
	pn.add_to_group("waypoints")
	
	# C1 (segunda passagem) - Rota B, índice 1 (3º no total)
	var c1_b = _waypoint("C1_B", Vector3(0, 0.5, 0), Color(1.0, 1.0, 0.0), 2, 1)  # Rota B, índice 1
	c1_b.add_to_group("waypoints")
	
	# BC (Bloco Comercial) - desvio da Rota B
	var bc = _waypoint("BC", Vector3(BLOCK_SIZE + STREET_WIDTH, 0.5, -BLOCK_SIZE - STREET_WIDTH), Color(1.0, 0.5, 0.0), 2, 2)  # Rota B, índice 2
	bc.add_to_group("waypoints")
	
	# C1 (terceira passagem) - Rota B, índice 2 (4º no total)
	var c1_c = _waypoint("C1_C", Vector3(0, 0.5, 0), Color(1.0, 1.0, 0.0), 2, 3)  # Rota B, índice 3
	c1_c.add_to_group("waypoints")
	
	# PG (retorno final) - fim da Rota B / round trip
	var pg_final = _waypoint("PG_Final", Vector3(0, 0.5, -BLOCK_SIZE - STREET_WIDTH), Color(0.5, 1.0, 0.5), 2, 4)  # Rota B, índice 4
	pg_final.add_to_group("waypoints")

func _waypoint(name: String, position: Vector3, color: Color, route_id: int, seq_index: int) -> Marker3D:
	var wp = Marker3D.new()
	wp.name = name
	wp.position = position
	
	# Script de detecção
	var script = Waypoint.new()
	script.waypoint_name = name
	script.detection_radius = 5.0
	script.route_id = route_id
	script.sequence_index = seq_index
	wp.set_script(script)
	
	# Visual (cilindro colorido)
	var visual = MeshInstance3D.new()
	visual.name = "Visual"
	visual.mesh = CylinderMesh.new()
	visual.mesh.radius = 2.0
	visual.mesh.height = 3.0
	var mat = StandardMaterial3D.new()
	mat.albedo_color = color
	mat.shading_mode = StandardMaterial3D.SHADING_MODE_UNSHADED
	visual.material = mat
	visual.position = Vector3(0, 1.5, 0)
	wp.add_child(visual)
	
	add_child(wp)
	return wp

func _setup_hud() -> void:
	var hud_scene = preload("res://scenes/hud.tscn")
	var hud = hud_scene.instantiate()
	add_child(hud)

func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed:
		match event.keycode:
			KEY_1:
				if not route_manager.is_active():
					route_manager.start_route(1)
			KEY_2:
				if not route_manager.is_active() and route_manager._route_a_done:
					route_manager.start_route(2)
			KEY_R:
				# Reset waypoints
				route_manager._reset_all_waypoints()
				print("[Grau Urbano] Waypoints resetados")

func _create_player() -> void:
	var player = CharacterBody3D.new()
	player.position = Vector3(-132, 1, 132)

	var col = CollisionShape3D.new()
	col.shape = CapsuleShape3D.new()
	col.shape.radius = 0.4
	col.shape.height = 1.8
	col.position = Vector3(0, 1, 0)
	player.add_child(col)

	var pivot = Node3D.new()
	pivot.position = Vector3(0, 1.7, 0)
	player.add_child(pivot)

	var cam = Camera3D.new()
	cam.position = Vector3(0, 0, -6)
	pivot.add_child(cam)

	var mesh = MeshInstance3D.new()
	mesh.mesh = CapsuleMesh.new()
	mesh.mesh.radius = 0.4
	mesh.mesh.height = 1.8
	mesh.material = _material(Color(0.8, 0.2, 0.2))
	mesh.position = Vector3(0, 0.9, 0)
	player.add_child(mesh)

	add_child(player)

	# Guarda referências para o script do player
	player.set_meta("camera_pivot", pivot)
	player.set_meta("camera", cam)

	var player_script = PlayerController.new()
	player_script.camera_pivot = pivot
	player_script.camera = cam
	player.set_script(player_script)

func _create_street(position: Vector3, size: Vector2) -> void:
	var street = MeshInstance3D.new()
	street.mesh = PlaneMesh.new()
	street.mesh.size = size
	street.position = position
	street.material = _material(Color(0.18, 0.18, 0.2))
	add_child(street)

func _building(position: Vector3, size: Vector3, color: Color) -> void:
	var building = MeshInstance3D.new()
	building.mesh = BoxMesh.new()
	building.mesh.size = size
	building.position = position + Vector3(0, size.y / 2.0, 0)
	building.material = _material(color)
	add_child(building)

func _marker(name: String, position: Vector3, color: Color) -> void:
	var marker = Marker3D.new()
	marker.name = name
	marker.position = position
	add_child(marker)

func _material(color: Color) -> StandardMaterial3D:
	var mat = StandardMaterial3D.new()
	mat.albedo_color = color
	mat.shading_mode = StandardMaterial3D.SHADING_MODE_UNSHADED
	return mat