type 'v env = (string * 'v) list

let rec lookup env x =
    match env with
    | [] -> failwith (Format.sprintf "%s not found" x)
    | (y, v) :: r ->
        if x = y then
          v
        else
          lookup r x

type 'v t = 'v env
