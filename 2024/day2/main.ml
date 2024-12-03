let read_file filename =
        let ic = open_in filename in 
        let rec read_lines acc =
                try
                        let line = input_line ic in 
                        let row = List.map int_of_string (String.split_on_char ' ' line) in
                        let () = List.iter (fun v -> Printf.printf "%d " v) row ; print_newline () in
                        read_lines (row :: acc)
                with
                | End_of_file -> close_in ic; List.rev acc
                | e -> close_in ic; raise e
        in 
        read_lines []

let generate_all_versions row =
        let rec remove_element item = function
                | [] -> []
                | x :: xs when x = item -> xs
                | x :: xs -> x :: remove_element item xs
        in
        List.fold_left (fun acc item -> remove_element item row :: acc ) [] row |> List.rev

let dist row =
        let rec windowing acc = function
                | [] | [_] -> acc
                | x :: y :: rest -> windowing (acc @ [x - y]) (y :: rest)
        in
        windowing [] row

let process_rows rows = 
        rows
        |> List.map (fun row ->
                        let check_asc = (fun d -> d <= -1 && d >= -3) in
                        let check_desc = (fun d -> d >= 1 && d <= 3) in
                        let all_versions = generate_all_versions row in
                        let is_correct_desc = List.exists (fun gen_row -> 
                                let dists = dist gen_row in
                                List.for_all check_desc dists
                        ) all_versions in
                        let is_correct_asc = List.exists (fun gen_row ->
                                let () = List.iter (fun r -> Printf.printf "%d " r) gen_row; print_newline () in
                                let dists = dist gen_row in
                                List.for_all check_asc dists
                        ) all_versions in
                        is_correct_desc ||is_correct_asc)
        |> List.map Bool.to_int
       (* |> List.map (fun row -> Printf.printf "%d\n" row; row)*)
        |> List.fold_left (+) 0


let () =
        let filename = "input.txt" in
        let data = read_file filename in
        let result = process_rows data in
        Printf.printf "Sum of safe reports = %d\n" result
