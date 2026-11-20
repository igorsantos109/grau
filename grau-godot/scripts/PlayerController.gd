extends CharacterBody3D

# Player — controle terceira pessoa (a pé), base para a moto na G02-F5
# Câmera segue atrás do player com rotação independente por mouse

@export var speed: float = 5.0
@export var mouse_sensitivity: float = 0.002

var camera_pivot: Node3D
var camera: Camera3D
var yaw: float = 0.0
var pitch: float = 0.0

func _ready() -> void:
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)

func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion:
		yaw -= event.relative.x * mouse_sensitivity
		pitch = clamp(pitch - event.relative.y * mouse_sensitivity, -PI / 2.1, PI / 2.1)

func _physics_process(delta: float) -> void:
	# Rotação do player (yaw)
	rotation.y = yaw

	# Movimento no plano XZ
	var input_dir = Vector2(
		Input.get_action_strength("move_right") - Input.get_action_strength("move_left"),
		Input.get_action_strength("move_backward") - Input.get_action_strength("move_forward")
	)
	input_dir = input_dir.normalized()

	var velocity = Vector3(input_dir.x, 0, input_dir.y) * speed
	velocity = velocity.rotated(Vector3(0, 1, 0), yaw)
	velocity = move_and_slide()

	# Aplica rotação da câmera (pitch clamped)
	if camera_pivot:
		camera_pivot.rotation.x = pitch
