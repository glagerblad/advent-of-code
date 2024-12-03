let read_file filename =
        let ic = open_in filename in 
        let rec read_lines acc =
                try
                        let line = input_line ic in 
                        let row = List.map int_of_string (String.split_on_char ' ' line) in
                        read_lines (row :: acc)
                with
                | End_of_file -> close_in ic; List.rev acc
                | e -> close_in ic; raise e
        in 

        read_lines []
        (* 10 1 2 3 4*)
let is_correct row =
        let rec check_ascending has_skipped first_pass = function
                | [] | [_] -> true
                | x :: y :: z :: rest ->
                        if (y - x) >= 1 && (y - x) <= 3 then check_ascending has_skipped false (y :: z :: rest) 
                        else if has_skipped then false
                        else if (z - x) >= 1 && (z - x) <= 3 then check_ascending true false (z :: rest)
                        else if first_pass && (z - y) >= 1 && (z - y) <= 3 then check_ascending true false (z :: rest)
                        else false
                | x :: y :: [] -> ((y - x) >= 1 && (y - x) <= 3) || not has_skipped
        in
        let rec check_descending has_skipped first_pass = function
                | [] | [_] -> true
                | x :: y :: z :: rest ->
                        if (x - y) >= 1 && (x - y) <= 3 then check_descending has_skipped false (y :: z :: rest) 
                        else if has_skipped then false
                        else if (x - z) >= 1 && (x - z) <= 3 then check_descending true false (z :: rest)
                        else if first_pass && (y - z) >= 1 && (y - z) <= 3 then check_descending true false (z :: rest)
                        else false
                | x :: y :: [] -> ((x - y) >= 1 && (x - y) <= 3) || not has_skipped
        in
        check_ascending false true row || check_descending false true row

let process_rows rows = 
        rows
        |> List.map (fun row -> if is_correct row then 1 else 0)
        |> List.map (fun n -> Printf.printf "%d " n; print_newline (); n)
        |> List.fold_left (+) 0

let () =
        let filename = "input.txt" in
        let data = read_file filename in
        let result = process_rows data in
        Printf.printf "Sum of safe reports = %d\n" result
