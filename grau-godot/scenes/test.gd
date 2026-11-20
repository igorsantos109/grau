extends Node
func _ready():
    var player = preload("res://scripts/player.gd").new()
    print("Loaded player script")
    get_tree().quit()
