# Custom Game Server for the Microservice Dungeon Game

This repository implements a Game Server in Rust for the [Microservice Dungeon Game](https://www.archi-lab.io/compounds/dungeon_main.html).

This Game service does not implement a round time, instead it automatically starts the next round once every player submitted a command for themselves and their robots.

## Key Features

- Multiple Games can run in parallel, for faster Deep learning
- Really similar API to the original Microservice Dungeon
- Time savings due to the removal of round times.

## Quickstart

Run `docker-compose up -d` in your Terminal. The Game Service will be available under `127.0.0.1:8080`

## Side Notes
This Game Server was mainly written, to empower Deep Learning for the [Microservice Dungeon Project](https://www.archi-lab.io/compounds/dungeon_main.html).

You can find my own Deep Learning Integration in [this Repository](https://www.archi-lab.io/compounds/dungeon_main.html](https://github.com/Amueller36/Deep-Q-Learning-Robot-Dungeon).

## High Level API Overview
## API Documentation

The Game Server provides the following API endpoints:

### Game Management

- `POST /games`: Create a new game
  - Request Body: `{ "max_rounds": number, "max_players": number, "map_size": number }`
  - Response: `{ "game_id": string }`

- `DELETE /games/{game_id}`: Delete a specific game
  - Response: `{ "game_id": string }`

- `GET /games`: Get all games
  - Response: List of GameState objects

- `GET /games/created`: Get all created games
  - Response: List of GameState objects for games in "Created" status

- `GET /games/{game_id}`: Get a specific game
  - Response: GameState object

- `GET /games/{game_id}/currentRound`: Get the current round of a specific game
  - Response: GameState object with only the current round

- `DELETE /games`: Delete all games
  - Response: List of deleted game IDs

### Player Management

- `GET /games/{game_id}/players`: Get all players in a game
  - Response: `{ "participating_players": [string] }`

- `PUT /games/{game_id}`: Join a game
  - Request Body: `{ "player_name": string }`
  - Response: `{ "player_name": string, "game_id": string, "money": number }`

### Game Commands

- `POST /games/{game_id}/gameCommands/start`: Start a game
  - Response: `{ "game_id": string, "game_status": string }`

- `POST /games/{game_id}/gameCommands/end`: End a game
  - Response: `{ "game_id": string, "game_status": string }`

### Map Display

- `GET /games/{game_id}/map`: Display the map for the current round
  - Response: String representation of the map

- `GET /games/{game_id}/map/rounds/{round_number}`: Display the map for a specific round
  - Response: String representation of the map

- `GET /games/{game_id}/map/players/{player_name}`: Display the map for a specific player
  - Response: String representation of the map

- `GET /games/{game_id}/map/rounds/{round_number}/players/{player_name}`: Display the map for a specific round and player
  - Response: String representation of the map

### Player and Robot Information

- `GET /games/{game_id}/currentRound/players/{player_name}/robots`: Get robots for the current round
  - Response: List of Robot objects

- `GET /games/{game_id}/currentRound/players/{player_name}/robots/{robot_id}`: Get a specific robot for the current round
  - Response: Robot object

- `GET /games/{game_id}/currentRound/players/{player_name}`: Get player state for the current round
  - Response: PlayerStateDto object

- `GET /games/{game_id}/currentRound/players/{player_name}/new`: Get detailed player state for the current round
  - Response: PlayerStateDto object with additional information

- `GET /games/{game_id}/{round_number}/players/{player_name}/new`: Get detailed player state for a specific round
  - Response: PlayerStateDto object with additional information

### Command Handling

- `POST /games/{game_id}/commands`: Handle a batch of commands
  - Request Body: List of Command objects
  - Response: 200 OK if successful, or appropriate error status

- `POST /games/{game_id}/commands/hypothetically`: Handle a batch of commands hypothetically
  - Request Body: List of Command objects
  - Response: PlayerStateDto object representing the hypothetical game state after applying the commands
