#!/bin/sh

INFILE="plik.bin"
MUTFILE="${INFILE}.fuz"

MUTATOR=mutator
PARSER=parser

while true; do
  ${MUTATOR} "${INFILE}" "${MUTFILE}" || exit 1
  ${PARSER} "${MUTFILE}" || exit 2
done
