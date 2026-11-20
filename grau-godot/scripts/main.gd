extends Node3D

# Cidade procedimental — Grau Urbano (G02-F2)
# Planta: dois quarteirões ligados por ruas com cruzamento X

const BLOCK_SIZE = 60.0      # lado do quarteirão (m)
const STREET_WIDTH = 12.0    # largura da rua (m)

func _ready() -> void:
	generate_city()
	_create_player()
	print("[Grau Urbano] Cidade gerada: 2 quarteirões, cruzamento X, SP/PR1/PR2 marcados")
	print("[Grau Urbano] Player instanciado em SP (-132, 1, 132)")

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

	# Marcadores
	_marker("SP", Vector3(-BLOCK_SIZE - STREET_WIDTH, 0.5, BLOCK_SIZE + STREET_WIDTH), Color(0, 0, 1))   # inferior esquerdo
	_marker("PR1", Vector3(BLOCK_SIZE + STREET_WIDTH, 0.5, -BLOCK_SIZE - STREET_WIDTH), Color(0, 1, 0)) # superior direito
	_marker("PR2", Vector3(BLOCK_SIZE + STREET_WIDTH, 0.5, BLOCK_SIZE + STREET_WIDTH), Color(1, 0, 0))   # inferior direito
	_marker("X", Vector3(0, 0.5, 0), Color(1, 1, 0))                                                    # cruzamento

	print("[Grau Urbano] SP=(-132, 0.5, 132), PR1=(132, 0.5, -132), PR2=(132, 0.5, 132), X=(0,0.5,0)")

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
