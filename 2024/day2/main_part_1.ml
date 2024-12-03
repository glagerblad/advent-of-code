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

let is_correct row =
        let rec check_ascending mistake = function
                | [] | [_] -> true
                | x :: y :: z :: rest ->
                        let () = Printf.printf "asc :: %B :: x: %d y: %d z: %d\n" mistake x y z in
                        if (y - x) >= 1 && (y - x) <= 3 then check_ascending mistake (y :: z :: rest) 
                        else 
                                if mistake then false
                                else if (z - x) >= 1 && (z - x) <= 3 then check_ascending true (z :: rest)
                                else false
                | x :: y :: rest ->
                                let () = Printf.printf "asc :: %B :: x: %d y: %d\n" mistake x y in
                                if (y - x) >= 1 && (y - x) <= 3 then check_ascending mistake (y :: rest) 
                                else false
        in
        let rec check_descending mistake = function
                | [] | [_] -> true
                | x :: y :: z :: rest ->
                        let () = Printf.printf "desc: :: %B  && %B :: %B :: x: %d y: %d z: %d\n" mistake ((x - y) >= 1) ((x - y) <= 3) x y z in
                        if (x - y) >= 1 && (x - y) <= 3 then
                                check_ascending mistake (y :: z :: rest) 
                        else 
                                if mistake then false
                                else if (x - z) >= 1 && (x - z) <= 3 then check_ascending true (z :: rest)
                                else false
                | x :: y :: rest ->
                        let () = Printf.printf "desc :: %B ::  x: %d y: %d\n" mistake x y in
                        if (x - y) >= 1 && (x - y) <= 3 then check_ascending mistake (y :: rest) 
                        else false
        in
        check_ascending false row  || check_descending false row

let process_rows rows = 
        rows
        |> List.map (fun row -> if is_correct row then 1 else 0)
        |> List.map (fun row -> (fun n -> Printf.printf "%d " n ) row; print_newline (); row)
        |> List.fold_left (+) 0

let () =
        let filename = "input.txt" in
        let data = read_file filename in
        let result = process_rows data in
        let () = List.iter (fun row -> List.iter (Printf.printf "%d ") row; print_newline ()) data in
        Printf.printf "Sum of correct reports = %d\n" result
