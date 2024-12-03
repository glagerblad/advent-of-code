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
                        let ds = dist row in 
                        let is_correct check = List.for_all check ds in 
                        is_correct check_asc || is_correct check_desc)
        |> List.map Bool.to_int
        |> List.fold_left (+) 0


let () =
        let filename = "input.txt" in
        let data = read_file filename in
        let result = process_rows data in
        Printf.printf "Sum of safe reports = %d\n" result
