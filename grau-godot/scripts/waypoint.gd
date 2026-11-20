extends Marker3D

# Waypoint de rota — detecta quando o player entra no raio e emite sinal

@export var waypoint_name: String = ""
@export var detection_radius: float = 5.0
@export var route_id: int = 0          # 1 = Rota A, 2 = Rota B
@export var sequence_index: int = 0    # ordem na sequência da rota

signal waypoint_reached(waypoint_name: String, route_id: int, sequence_index: int)

var _player: Node3D = null
var _reached: bool = false

func _ready() -> void:
	# Procura o player na árvore
	_player = get_parent().get_node_or_null("Player")
	if not _player:
		# Tenta achar pelo grupo
		for node in get_tree().get_nodes_in_group("player"):
			_player = node
			break

func _process(delta: float) -> void:
	if _reached or not _player:
		return
	
	var dist = global_position.distance_to(_player.global_position)
	if dist <= detection_radius:
		_reached = true
		waypoint_reached.emit(waypoint_name, route_id, sequence_index)
		# Feedback visual
		if has_node("Visual"):
			get_node("Visual").visible = false

func reset() -> void:
	_reached = false
	if has_node("Visual"):
		get_node("Visual").visible = true