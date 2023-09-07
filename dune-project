(lang dune 3.7)
(using menhir 2.0)

(name swallow)

(generate_opam_files true)

(source
 (github muqiuhan/swallow))

(authors "Muqiu Han")

(maintainers "Muqiu Han")

(license MIT)

(documentation https://github.com/muqiuhan/swallow)

(package
 (name swallow)
 (synopsis "")
 (description "")
 (depends ocaml dune menhir alcotest)
 (tags
  (type language)))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/dune-files.html#dune-project
