//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v2                                                  //
//  Modified:  31dec24                                            //
//---------------------------------------------------------------//
// DESCRIPCIÓN:                                                
// Componente de un juego de super tres en raya.



//-------------------------------------------------------------------
// IMPORTS



use yew::prelude::*;
use std::collections::HashSet;



//-------------------------------------------------------------------
//-------------------------------------------------------------------
// DATA STRUCTURES


/// Enumeración que representa a los jugadores
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    First,
    Second,
}

/// Función que cambia de jugador
fn switch(op: Option<Player>) -> Option<Player> {
    match op {
        Some(Player::First) => Some(Player::Second),
        Some(Player::Second) => Some(Player::First),
        None => None,
    }
}

/// Estructura que representa una posición en el tablero
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {

    /// Proporciona la posición total en el supertablero a partir de la posición en un subtablero
    fn total_pos(pos1: Position, pos2: Position) -> Position {
        Position { x: pos1.x * 3 + pos2.x, y: pos1.y * 3 + pos2.y }
    }

    fn partial_pos(total_pos: Position) -> (Position, Position) {
        (Position { x: total_pos.x / 3, y: total_pos.y / 3 }, Position { x: total_pos.x % 3, y: total_pos.y % 3 })
    }
}

/// Posición inalcanzable.
/// 
/// X = 3, Y = 3
/// 
const UNREACHABLE: Position = Position { x: 9, y: 9 };

/// Estructura que representa un tablero de 3x3
#[derive(Clone, PartialEq, Debug)]
struct TableroTres {
    data: [[Option<Player>; 3]; 3],
}

impl TableroTres {

    fn new() -> Self {
        Self {
            data: [[None; 3]; 3],
        }
    }

    fn put(&mut self, pos: Position, value: Player) {
        self.data[pos.x as usize][pos.y as usize] = Some(value);
    }

    fn reset(&mut self, pos: Position) {
        self.data[pos.x as usize][pos.y as usize] = None;
    }

    fn get(&self, pos: Position) -> Option<Player> {
        self.data[pos.x as usize][pos.y as usize]
    }

    fn check(&self) -> Option<Player> {

        let check_line = |line: [Option<Player>; 3]| -> Option<Player> {
            if line.iter().all(|x| x.is_some()) && line[0] == line[1] && line[1] == line[2] {
                // Si todos los elementos son iguales y no son None, devuelvo el valor, el tablero se completó
                line[0]
            } else {
                None
            }
        };
    
        // Check rows
        for i in 0..3 {
            if let Some(r) = check_line(self.data[i]) {
                return Some(r);
            }
        }
    
        // Check columns
        for i in 0..3 {
            if let Some(r) = check_line([self.data[0][i], self.data[1][i], self.data[2][i]]) {
                return Some(r);
            }
        }
    
        // Check diagonals
        if let Some(r) = check_line([self.data[0][0], self.data[1][1], self.data[2][2]]) {
            return Some(r);
        }
        if let Some(r) = check_line([self.data[0][2], self.data[1][1], self.data[2][0]]) {
            return Some(r);
        }
    
        None
    }

    fn playable(&self) -> bool {
        self.data.iter().any(|x| x.iter().any(|y| y.is_none()))
    }


}


/// Estructura que representa un supertablero de 3x3
pub struct TableroSuperTres {
    tablero: [[Result<Player, TableroTres>; 3]; 3],
}

impl TableroSuperTres {

    fn new() -> Self {

        let tablero = [
            [Err(TableroTres::new()), Err(TableroTres::new()), Err(TableroTres::new())],
            [Err(TableroTres::new()), Err(TableroTres::new()), Err(TableroTres::new())],
            [Err(TableroTres::new()), Err(TableroTres::new()), Err(TableroTres::new())],
        ];

        Self {
            tablero,
        }
    }

    fn get(&self, pos1: Position) -> Result<&Player, &TableroTres> {
        self.tablero[pos1.x as usize][pos1.y as usize].as_ref()
    }

    fn get_mut(&mut self, pos1: Position) -> Result<&mut Player, &mut TableroTres> {
        self.tablero[pos1.x as usize][pos1.y as usize].as_mut()
    }

    fn put(&mut self, pos1: Position, pos2: Position, value: Player) {
        let tab = self.get_mut(pos1);

        match tab {
            Err(tab3x3) => {
                // Si tengo un tablero, lo modifico
                tab3x3.put(pos2, value);
            },
            Ok(_) => {
                // Si tengo un valor, el tablero ya se completó, no hago nada
            },
        }
        
    }

    fn reset(&mut self, pos1: Position, pos2: Position) {
        let tab = self.get_mut(pos1);

        match tab {
            Err(tab3x3) => {
                // Si tengo un tablero, lo modifico
                tab3x3.reset(pos2);
            },
            Ok(_) => {
                // Si tengo un valor, el tablero ya se completó, no hago nada
            },
        }
    }

    /// Chequea si el supertablero está completo.
    /// Si el supertablero está completo, devuelve el valor.
    /// 
    /// Los subtableros se chequean y si están completos, se cambian por el valor
    fn check(&mut self) -> Option<Player> {

        // Primero chequeo cada subtablero por si está completo y sustituyo dicho subtablero del supertablero
        // por el valor ganador
        for i in 0..3 {
            for j in 0..3 {
                if let Err(tab) = &self.tablero[i][j] {
                    if let Some(value) = tab.check() {
                        self.tablero[i][j] = Ok(value);
                    }
                }
            }
        }

        // Después, chequeo si el supertablero está completo y devuelvo el valor ganador

        let check_line = |line: &[Result<Player, TableroTres>; 3]| -> Option<Player> {
            if line.iter().all(|x| x.is_ok()) {
                let first_value = line[0].as_ref().ok()?;
                if line.iter().all(|x| x.as_ref().ok() == Some(first_value)) {
                    // Si todos los elementos son iguales y no son None, devuelvo el valor, el tablero se completó
                    return Some(*first_value);
                }
            }
            None
        };

        // Check rows
        for i in 0..3 {
            if let Some(r) = check_line(&self.tablero[i]) {
                return Some(r);
            }
        }

        // Check columns
        for i in 0..3 {
            if let Some(r) = check_line(&[self.tablero[0][i].clone(), self.tablero[1][i].clone(), self.tablero[2][i].clone()]) {
                return Some(r);
            }
        }

        // Check diagonals
        if let Some(r) = check_line(&[self.tablero[0][0].clone(), self.tablero[1][1].clone(), self.tablero[2][2].clone()]) {
            return Some(r);
        }
        if let Some(r) = check_line(&[self.tablero[0][2].clone(), self.tablero[1][1].clone(), self.tablero[2][0].clone()]) {
            return Some(r);
        }

        None

    }

}


//-------------------------------------------------------------------
//-------------------------------------------------------------------
// COMPONENT


/// Mensajes que puede recibir el componente
pub enum SuperTresMsg {
    Mark(Position, Position),
    Check,
}


/// Información que guarda el componente
pub struct SuperTresComponent {

    /// Supertablero de juego
    tablero: TableroSuperTres,

    /// Jugador al que le toca jugar.
    /// Si es None, el juego terminó.
    turn: Option<Player>,

    /// Indica si se jugó en el último turno.
    turn_played: bool,

    /// Última casilla jugada
    last_played: Option<Position>,

    /// Conjunto de posiciones jugadas.
    /// Las posiciones jugadas no se pueden volver a jugar.
    /// 
    /// Son posiciones totales en el supertablero, NO se refieren a subtableros.
    played_total_positions: HashSet<Position>,

    /// Última posición total jugada.
    last_total_played: Option<Position>,

    /// Subtablero activo.
    /// Si es None, cualquier subtablero está activo.
    active_table: Option<Position>,

    /// Ganador
    winner: Option<Player>,
}


/// Implementación del componente
impl Component for SuperTresComponent {


    type Message = SuperTresMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        
        Self { 
            tablero: TableroSuperTres::new(),
            turn: Some(Player::First),
            turn_played: false,
            last_played: None,
            played_total_positions: HashSet::new(),
            last_total_played: None,
            active_table: Some(Position { x: 1, y: 1 }),
            winner: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            SuperTresMsg::Mark(position1, position2) => {

                // Compruebo si el juego continúa
                if let Err(tab) = self.tablero.get(position1) {

                    // El juego continúa, marco la casilla

                    match self.turn {

                        // Turno del jugador 1
                        Some(Player::First) => {
                            
                            // Compruebo la casilla que se quiere marcar
                            match tab.get(position2) {
                                None => {

                                    marcar_casilla_libre(self, position1, position2, Player::First)

                                },
                                Some(Player::First) => {

                                    marcar_casilla_ocupada(self, position1, position2)
                                    
                                },
                                Some(Player::Second) => {
                                    // Si ya está marcado por el otro jugador, no se hace nada
                                    false
                                }
                            }

                        },
                        // Turno del jugador 2
                        Some(Player::Second) => {
                            
                            // Compruebo la casilla que se quiere marcar
                            match tab.get(position2) {
                                None => {

                                    marcar_casilla_libre(self, position1, position2, Player::Second)

                                },
                                Some(Player::Second) => {

                                    marcar_casilla_ocupada(self, position1, position2)

                                },
                                Some(Player::First) => {
                                    // Si ya está marcado por el otro jugador, no se hace nada
                                    false
                                }
                            }
                        },
                        // Ningún jugador
                        None => {
                            // El juego ya terminó, no se hace nada
                            false
                        },
                    }

                } else {

                    // El juego ya terminó, no se hace nada

                    false
                }

            },
            SuperTresMsg::Check => {

                let final_result = self.tablero.check();

                match final_result {
                    None => {
                        
                        // El juego continúa

                        // Si no se jugó, no se hace nada
                        if self.turn_played {
                
                            // El subtablero en el que quiero jugar es el que corresponde a la última casilla jugada
                            let future_table = self.tablero.get(self.last_played.unwrap());
                            
                            if let Err(future_table) = future_table {
                                // Si el tablero no está completo, el tablero activo es el subtablero correspondiente
                                if future_table.playable() {
                                    self.active_table = self.last_played;
                                } else {
                                    // Si el tablero no tiene casillas libres y no es completo,
                                    // es un empate y el tablero activo es el super tablero completo
                                    self.active_table = None;
                                }
                            } else {
                                // Si el tablero está completo, el tablero activo es el supertablero completo
                                self.active_table = None;
                            }

                            // Cambio de turno
                            self.turn = switch(self.turn);
                            self.turn_played = false;

                            // Se almacena la posición total jugada y se limpia la última posición total jugada
                            self.played_total_positions.insert(self.last_total_played.unwrap());
                            self.last_total_played = None;

                        }

                    },
                    Some(value) => {

                        // El juego terminó

                        // Si el supertablero está completo, el tablero activo es inalcanzable.
                        // El turno es None.
                        // El ganador es el valor del supertablero.

                        self.active_table = Some(UNREACHABLE);
                        self.turn = None;
                        self.winner = Some(value);
                        self.turn_played = false;
                    },
                }

                true
            },
            
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let winner_class = match self.winner {
            Some(Player::First) => "bg-first-player text-white",
            Some(Player::Second) => "bg-second-player text-white",
            None => ""
        };

        html! {
            <div class={classes!("section", "game-section", winner_class)}>
                
                <div class={"container is-flex is-justify-content-center board-container"}>

                    <div class={classes!(
                        "box", 
                        "game-board",
                        {
                            if let Some(winner) = self.winner {
                                match winner {
                                    Player::First => "back-fp",
                                    Player::Second => "back-sp",
                                }
                            } else {
                                ""
                            }
                        }
                    )}>

                        // Botón de siguiente turno
                        <div class="block gb-up">
                            <button 
                                class={classes!("button", "is-high", "is-fullwidth")}
                                onclick={ctx.link().callback(move |_| SuperTresMsg::Check)}
                                disabled={!self.turn_played}
                            >
                                {"NEXT"}
                            </button>
                        </div>

                        // Tablero de juego
                        <div class="block gb-mid">
                            { self.render_super_board(ctx) }
                        </div>

                        // Anuncio de ganador
                        <div class="block gb-bot">
                            {
                                if let Some(winner) = self.winner {
                                    html! {
                                        <div class="title is-4 has-text-centered">
                                            <div>
                                                {format!("Player {} wins!", 
                                                    match winner {
                                                        Player::First => "X",
                                                        Player::Second => "O"
                                                    }
                                                )}
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                        
                    </div>
                </div>
            </div>
        }
    }
}

impl SuperTresComponent {

    fn render_super_board(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container is-flex is-justify-content-center">
            <div class="super-grid columns is-centered">
                <div class="column is-narrow">
                    { for (0..3).map(|i| self.render_super_row(ctx, i)) }
                </div>
            </div>
        </div>
        }
    }


    fn render_super_row(&self, ctx: &Context<Self>, i: u8) -> Html {
        html! {
            <div class="columns super-row" key={i}>
                { for (0..3).map(|j| self.render_super_cell(ctx, i, j)) }
            </div>
        }
    }


    fn render_super_cell(&self, ctx: &Context<Self>, i: u8, j: u8) -> Html {
        let position = Position { x: i, y: j };
        let is_active = !disabled_table(self.active_table, position);
        
        let cell_classes = classes!(
            "super-cell",
            "column",
            "is-narrow",
            if is_active { "active-board" } else { "inactive-board" }
        );

        html! {
            <div class={cell_classes} key={format!("{}-{}", i, j)}>
                {
                    match self.tablero.get(position) {
                        Ok(Player::First) => html! {
                            <div class="won-cell first-player is-flex is-justify-content-center is-align-items-center">{"X"}</div>
                        },
                        Ok(Player::Second) => html! {
                            <div class="won-cell second-player is-flex is-justify-content-center is-align-items-center">{"O"}</div>
                        },
                        Err(tab) => self.render_sub_board(ctx, tab, position),
                    }
                }
            </div>
        }
    }


    fn render_sub_board(&self, ctx: &Context<Self>, tab: &TableroTres, position: Position) -> Html {
        html! {
            <div class="sub-grid">
                { for (0..3).map(|k| self.render_sub_row(ctx, tab, position, k)) }
            </div>
        }
    }


    fn render_sub_row(&self, ctx: &Context<Self>, tab: &TableroTres, pos1: Position, k: u8) -> Html {
        html! {
            <div class="columns is-gapless sub-row" key={k}>
                { for (0..3).map(|l| self.render_sub_cell(ctx, tab, pos1, k, l)) }
            </div>
        }
    }


    fn render_sub_cell(&self, ctx: &Context<Self>, tab: &TableroTres, pos1: Position, k: u8, l: u8) -> Html {
        let pos2 = Position { x: k, y: l };
        let is_disabled = disabled_table(self.active_table, pos1);
        let total_pos = Position::total_pos(pos1, pos2);
        let is_played = self.played_total_positions.contains(&total_pos);

        let cell_classes = classes!(
            "button",
            match tab.get(pos2) {
                Some(Player::First) => "btn-fp",
                Some(Player::Second) => "btn-sp",
                None => "btn-none"
            },
            if is_played { "played" } else { "unplayed" }
        );

        html! {
            <div class="column is-narrow sub-cell">
                <button
                    class={cell_classes}
                    disabled={is_disabled}
                    onclick={ctx.link().callback(move |_| SuperTresMsg::Mark(pos1, pos2))}
                >
                    {
                        match tab.get(pos2) {
                            Some(Player::First) => "X",
                            Some(Player::Second) => "O",
                            None => " "
                        }
                    }
                </button>
            </div>
        }
    }

}


fn disabled_table(active_table: Option<Position>, actual_table: Position) -> bool {
    (active_table.is_some())&&(active_table.unwrap() != actual_table)
}


fn marcar_casilla_libre(super_tres: &mut SuperTresComponent, pos1: Position, pos2: Position, player: Player) -> bool {
    if super_tres.turn_played {
        // Si ya se jugó y no se hizo check, se revierte la jugada anterior
        // para que no puedas marcar varias casillas en un solo turno
        let (last_table, _) = Position::partial_pos(super_tres.last_total_played.unwrap());
        super_tres.tablero.reset(last_table, super_tres.last_played.unwrap());
        //self.last_total_played = None;
    }

    // Si no está marcado, se marca
    // El siguiente tablero activo es el subtablero correspondiente a la casilla marcada
    // Se jugó el turno

    super_tres.tablero.put(pos1, pos2, player);
    super_tres.last_played = Some(pos2);
    // Se guarda la posición total jugada
    super_tres.last_total_played = Some(Position::total_pos(pos1, pos2));
    super_tres.turn_played = true;

    true
}


fn marcar_casilla_ocupada(super_tres: &mut SuperTresComponent, pos1: Position, pos2: Position) -> bool {
    // Compruebo si la casilla ya fue jugada por el mismo jugador anteriormente
    if !super_tres.played_total_positions.contains(&Position::total_pos(pos1, pos2)) {
                                        
        // Si ya está marcado, se resetea
        // Si se resetea, se vuelve a jugar en el mismo tablero
        // No se jugó el turno

        super_tres.tablero.reset(pos1, pos2);
        super_tres.last_played = Some(pos1);
        super_tres.turn_played = false;
        // No hay posición total jugada
        super_tres.last_total_played = None;

        true
    } else {
        // Si ya está marcado anteriormente por el mismo jugador, no se hace nada
        false
    }
}
