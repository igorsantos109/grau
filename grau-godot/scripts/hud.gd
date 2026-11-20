extends CanvasLayer

# HUD urbano — G02-F3
# Mostra rota ativa, progresso, waypoints

@onready var route_label = $RouteLabel
@onready var progress_bar = $ProgressBar
@onready var progress_label = $ProgressLabel
@onready var waypoint_list = $WaypointList

var _route_manager: RouteManager = null

func _ready() -> void:
	# Busca o RouteManager na cena principal
	_route_manager = get_tree().root.get_node_or_null("Main/RouteManager")
	if not _route_manager:
		# Tenta pelo grupo
		for node in get_tree().get_nodes_in_group("route_manager"):
			_route_manager = node
			break
	
	if _route_manager:
		_route_manager.route_started.connect(_on_route_started)
		_route_manager.waypoint_crossed.connect(_on_waypoint_crossed)
		_route_manager.route_completed.connect(_on_route_completed)
	
	# Estado inicial
	route_label.text = "ROTA: Nenhuma (1=Rota A, 2=Rota B)"
	progress_bar.value = 0
	progress_label.text = "0/0"
	waypoint_list.text = ""

func _process(delta: float) -> void:
	if _route_manager and _route_manager.is_active():
		progress_bar.value = _route_manager.get_progress_ratio() * 100
		var current = _route_manager.get_current_route_name()
		route_label.text = "ROTA ATIVA: " + current
		# progress_label atualizado via signal

func _on_route_started(route_name: String) -> void:
	route_label.text = "ROTA INICIADA: " + route_name
	progress_bar.value = 0
	waypoint_list.text = ""

func _on_waypoint_crossed(name: String, route_id: int, progress: String) -> void:
	progress_label.text = progress
	waypoint_list.text += "✓ " + name + "\n"

func _on_route_completed(route_name: String, round_trip: bool, total_time: float) -> void:
	route_label.text = "✅ " + route_name + " CONCLUÍDO!" + (", Round Trip: %.1fs" % [total_time] if round_trip else "")
	progress_bar.value = 100