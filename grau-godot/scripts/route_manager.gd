extends Node

# Gerenciador de rotas urbanas — G02-F3
# Rota A (ida): PG(0) → C1(1) → PN(2)
# Rota B (volta): PN(2) → C1(3) → BC(4) → C1(5) → PG(6)
# Round trip completo: sequência de 7 waypoints

signal route_started(route_name: String)
signal waypoint_crossed(waypoint_name: String, route_id: int, progress: String)
signal route_completed(route_name: String, round_trip: bool, total_time: float)

# Definição das rotas
const ROUTE_A_NAME = "Rota A (Norte: PG→C1→PN)"
const ROUTE_B_NAME = "Rota B (Sul: PN→C1→BC→C1→PG)"
const ROUTE_A_WAYPOINTS = ["PG", "C1", "PN"]
const ROUTE_B_WAYPOINTS = ["PN", "C1", "BC", "C1", "PG"]
const FULL_SEQUENCE = ["PG", "C1", "PN", "C1", "BC", "C1", "PG"]  # 7 waypoints para round trip

# Estado
var _current_route: int = 0           # 0 = none, 1 = Rota A, 2 = Rota B
var _current_index: int = 0
var _start_time: float = 0.0
var _round_trip: bool = false
var _route_a_done: bool = false

# Referências
var _waypoints: Dictionary = {}

func _ready() -> void:
	# Registra waypoints por nome
	for node in get_tree().get_nodes_in_group("waypoints"):
		_waypoints[node.waypoint_name] = node
		node.waypoint_reached.connect(_on_waypoint_reached.bind())
	
	print("[RouteManager] Iniciado. Waypoints registados: ", _waypoints.keys())

func start_route(route_id: int) -> void:
	if route_id == 1:
		_current_route = 1
		_current_index = 0
		_route_a_done = false
		_round_trip = false
		_start_time = Time.get_ticks_msec() / 1000.0
		_reset_all_waypoints()
		route_started.emit(ROUTE_A_NAME)
		print("[RouteManager] Rota A iniciada: PG → C1 → PN")
	elif route_id == 2:
		_current_route = 2
		_current_index = 0
		_round_trip = true
		_start_time = Time.get_ticks_msec() / 1000.0
		_reset_all_waypoints()
		route_started.emit(ROUTE_B_NAME)
		print("[RouteManager] Rota B iniciada: PN → C1 → BC → C1 → PG")

func _reset_all_waypoints() -> void:
	for wp in _waypoints.values():
		wp.reset()

func _on_waypoint_reached(name: String, route_id: int, seq_index: int) -> void:
	if _current_route == 0:
		return
	
	# Verifica se é o waypoint esperado na sequência atual
	var expected_name = _get_expected_waypoint()
	if name != expected_name:
		# Waypoint fora de ordem - ignora ou reseta
		print("[RouteManager] Waypoint fora de ordem: ", name, " (esperado: ", expected_name, ")")
		return
	
	_current_index += 1
	var progress = _get_progress_string()
	waypoint_crossed.emit(name, _current_route, progress)
	print("[RouteManager] Waypoint cruzado: ", name, " | Progresso: ", progress)
	
	# Verifica conclusão
	if _current_route == 1 and _current_index >= ROUTE_A_WAYPOINTS.size():
		_route_a_done = true
		_current_route = 0
		print("[RouteManager] Rota A CONCLUÍDA! Aguardando início da Rota B...")
		# Auto-inicia Rota B após breve pausa
		call_deferred("_start_route_b_after_delay")
	elif _current_route == 2 and _current_index >= ROUTE_B_WAYPOINTS.size():
		var total_time = Time.get_ticks_msec() / 1000.0 - _start_time
		_current_route = 0
		route_completed.emit("Round Trip Completo", true, total_time)
		print("[RouteManager] ROUND TRIP COMPLETO! Tempo total: ", total_time, "s")

func _start_route_b_after_delay() -> void:
	await get_tree().create_timer(1.0).timeout
	if _route_a_done:
		start_route(2)

func _get_expected_waypoint() -> String:
	if _current_route == 1:
		if _current_index < ROUTE_A_WAYPOINTS.size():
			return ROUTE_A_WAYPOINTS[_current_index]
	elif _current_route == 2:
		if _current_index < ROUTE_B_WAYPOINTS.size():
			return ROUTE_B_WAYPOINTS[_current_index]
	return ""

func _get_progress_string() -> String:
	if _current_route == 1:
		return "%d/%d (Rota A)" % [_current_index, ROUTE_A_WAYPOINTS.size()]
	elif _current_route == 2:
		return "%d/%d (Rota B)" % [_current_index, ROUTE_B_WAYPOINTS.size()]
	return "0/0"

func get_current_route_name() -> String:
	if _current_route == 1:
		return ROUTE_A_NAME
	elif _current_route == 2:
		return ROUTE_B_NAME
	return "Nenhuma"

func get_progress_ratio() -> float:
	if _current_route == 1:
		return float(_current_index) / float(ROUTE_A_WAYPOINTS.size())
	elif _current_route == 2:
		return float(_current_index) / float(ROUTE_B_WAYPOINTS.size())
	return 0.0

func is_active() -> bool:
	return _current_route != 0

func is_round_trip() -> bool:
	return _round_trip