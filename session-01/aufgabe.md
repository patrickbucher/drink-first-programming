# Aufgabenstellung

Implementiere das Spiel _Vier Gewinnt_ für zwei menschliche Spieler als
Kommandozeilenanwendung in Rust. Die Spiellogik soll als Library umgesetzt und
mithilfe von Unittests geprüft werden. Der interaktive Teil kann manuell
getestet werden. Das Spiel soll auf einem Gitter von sieben (Breite) mal sechs
(Höhe) gespielt werden können. Der Spieler, der zuerst vier seiner Steine (`x`
und `o`) in eine ununterbrochene horizontale, vertikale oder diagonale Reihe
bringen kann, gewinnt das Spiel. Ist das Gitter komplett gefüllt, ohne dass
eine der Siegbedingungen eingetreten ist, wird das Spiel als Unentschieden
gewertet.
