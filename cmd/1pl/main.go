package main

import (
	"bytes"
	"context"
	"flag"
	"log"
	"runtime/debug"
	"strings"

	"github.com/ichiban/prolog/engine"
)

const (
	prompt          = "?- "
	contPrompt      = "|- "
	userInputPrompt = "|: "
)

var version = func() string {
	info, ok := debug.ReadBuildInfo()
	if !ok {
		return ""
	}

	return info.Main.Version
}()

func main() {
	input := strings.NewReader("Some prolog code")
	output := bytes.NewBuffer([]byte{})
	i := New(input, output)
	// 	i.Register1(engine.NewAtom("halt"), halt)
	i.Unknown = func(name engine.Atom, args []engine.Term, env *engine.Env) {
		var sb strings.Builder
		s := engine.NewOutputTextStream(&sb)
		_, _ = engine.WriteTerm(&i.VM, s, name.Apply(args...), engine.List(engine.NewAtom("quoted").Apply(engine.NewAtom("true"))), engine.Success, env).Force(context.Background())
		log.Printf("UNKNOWN %s", &sb)
	}

	// Consult arguments.
	if err := i.QuerySolution(`consult(?).`, flag.Args()).Err(); err != nil {
		log.Panic(err)
	}
}
