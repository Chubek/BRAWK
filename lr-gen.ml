type symbol =
  | Terminal of string
  | NonTerminal of string

type production = {
  head: string;
  body: symbol list;
  action: string option;
}

type action =
  | Shift of int
  | Reduce of production
  | Accept

type grammar = {
  specs: string;
  symbols: symbol list;
  parsing_table: action list list;
  productions: production list;
  start_symbol: string;
}

let generate_parsing_table (grammar : ref grammar) =
  let rec closure items =
    let rec closure_item_set item_set acc =
      match item_set with
      | [] -> acc
      | item :: rest ->
          if List.mem item acc then closure_item_set rest acc
          else
            let lookahead_set = match item.lookahead with Some s -> [s] | None -> [] in
            let next_items =
              match List.nth item.body item.dot_position with
              | Some (NonTerminal nt) ->
                  List.map
                    (fun prod ->
                      { production = prod; dot_position = 0; lookahead = None })
                    (List.filter (fun p -> p.head = nt) !grammar.productions)
              | _ -> []
            in
            let next_item_set =
              List.map (fun i -> { i with lookahead = None }) next_items @ lookahead_set
            in
            closure_item_set (rest @ next_item_set) (item :: acc)
    in
    closure_item_set items []
  in

  let rec goto items symbol =
    List.map
      (fun item ->
        match List.nth item.body item.dot_position with
        | Some s when s = symbol -> { item with dot_position = item.dot_position + 1 }
        | _ -> item)
      items
  in

  let rec construct_lr1_items closure_table transitions =
    match closure_table with
    | [] -> (List.rev transitions, [])
    | item_set :: rest ->
        let transitions, next_closure_table =
          List.fold_left
            (fun (trans, c_table) sym ->
              let goto_set = goto item_set sym in
              if goto_set = [] then (trans, c_table)
              else
                match List.find_opt (fun set -> List.sort compare set = List.sort compare goto_set) c_table with
                | Some existing_set ->
                    let idx = List.length c_table - 1 in
                    (Shift idx :: trans, c_table)
                | None ->
                    let idx = List.length c_table in
                    (Shift idx :: trans, goto_set :: c_table))
            (transitions, rest) grammar.symbols
        in
        let next_items = List.flatten (List.map (fun sym -> closure (goto item_set sym)) grammar.symbols) in
        let next_closure_table = if next_items = [] then next_closure_table else next_items :: next_closure_table in
        construct_lr1_items next_closure_table transitions
  in

  let rec construct_parsing_table grammar lr1_items transitions =
    List.map
      (fun item_set ->
        List.map
          (fun sym ->
            match List.find_opt (fun i -> List.sort compare (goto item_set sym) = List.sort compare i) lr1_items with
            | Some next_set ->
                if List.exists (fun i -> List.sort compare (goto item_set sym) = List.sort compare i) next_set then
                  Shift (List.length next_set - 1)
                else Reduce (List.find_opt (fun prod -> List.sort compare prod.body = List.sort compare (goto item_set sym)) grammar.productions)
            | None -> Accept)
          grammar.symbols)
      lr1_items
  in

  (* Initial LR(1) items *)
  let initial_item = { production = List.hd !grammar.productions; dot_position = 0; lookahead = Some "$" } in
  let initial_state = closure [initial_item] in

  (* Construct LR(1) items and transitions *)
  let lr1_items, transitions = construct_lr1_items [initial_state] [] in

  (* Construct the parsing table *)
  let parsing_table = construct_parsing_table !grammar lr1_items transitions in

  grammar := {!grammar with parsing_table}

let generate_main_parser (grammar : ref grammar) =
        (* implement the main parser logic *)

let get_definitions (grammar : ref grammar) =
        (* implement the code that gets the yacc-style definitions  *)

let get_tokens (grammar : ref grammar) =
        (* implement the code that gets the token declarations --- %token <TOKEN>  *)

let get_types (grammar : ref grammar) = 
        (* implement the code that gets the type declarations --- %type <type> <nonterminal>  *)

let get_user_code (grammar : ref grammar) =
        (* implement the code that gets the yacc-style user codes at the end of the specs *)

let get_start_symbol (grammar : ref grammar) =
        (* implement the code that gets the start symbol --- %start <nonterminal>  *)


