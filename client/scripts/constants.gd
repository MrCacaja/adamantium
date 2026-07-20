extends Node

enum Action {
	SyncEntity,
	SyncId,
	Disconnect,
	Chat
}

enum Movement {
	Idle,
	Walk
}

enum Direction {
	Down,
	Left,
	Right,
	Up
}

const SPRITES_CONFIGS = {
	"human": {
		"frame_size": Vector2i(32, 32),
		"padding": 16,
		"animations": {
			"idle": {
				"down": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Idle_Base/Idle_Down-Sheet.png",
					"frames": 4,
					"speed": 1.0
				},
				"side": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Idle_Base/Idle_Side-Sheet.png",
					"frames": 4,
					"speed": 1.0
				},
				"up": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Idle_Base/Idle_Up-Sheet.png",
					"frames": 4,
					"speed": 1.0
				}
			},
			"walk": {
				"down": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Walk_Base/Walk_Down-Sheet.png",
					"frames": 6,
					"speed": 1.0
				},
				"side": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Walk_Base/Walk_Side-Sheet.png",
					"frames": 6,
					"speed": 1.0
				},
				"up": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Walk_Base/Walk_Up-Sheet.png",
					"frames": 6,
					"speed": 1.0
				}
			}
		}
	}
}

const TICK_RATE_SECS = 0.2
const INTERP_DURATION = 0.08
const CHAT_RADIUS = 300.0
const CHAT_MAX_MESSAGES = 50
