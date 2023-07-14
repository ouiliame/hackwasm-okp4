//go:generate ../../../bin/tinyjson -all -snake_case contract.go
package src

import (
	"bytes"
	"errors"
	"strings"

	"github.com/CosmWasm/cosmwasm-go/std"
	"github.com/CosmWasm/cosmwasm-go/std/types"
	"github.com/ichiban/prolog"
)

// FirstKey defines the value of the default key,
// when no key is set in the contract so far.
// NOTE: keys are [1]byte length but in KV they're [n]bytes.
const FirstKey byte = 0

// ExecuteMsg defines all the messages that modify state that can be sent to the contract.
type ExecuteMsg struct {
	PrologStatement string `json:"prolog_statement"`
}

// QueryMsg defines all the set of the possible queries that can be sent to the contract.
type QueryMsg struct {
	PrologStatement string `json:"prolog_statement"`
}

// InstantiateMsg is the instantiation messages.
type InstantiateMsg struct{}

// Instantiate does nothing.
func Instantiate(_ *std.Deps, _ types.Env, _ types.MessageInfo, _ []byte) (*types.Response, error) {
	return &types.Response{}, nil
}

// Execute runs state modifying handlers of the contract given msg data.
func Execute(deps *std.Deps, env types.Env, info types.MessageInfo, data []byte) (*types.Response, error) {
	msg := ExecuteMsg{}
	err := msg.UnmarshalJSON(data)
	if err != nil {
		return nil, err
	}

	switch {
	case msg.PrologStatement != "":
		return executeProlog(deps, env, info, &msg.PrologStatement)
	}
	return nil, errors.New("unknown request") // TODO(fdymylja): make this a common error in some package once we sort out devex on ExecuteMsg
}

func executeProlog(deps *std.Deps, _ types.Env, _ types.MessageInfo, msg *string) (*types.Response, error) {
	iter := deps.Storage.Range(nil, nil, std.Ascending)
	resp := &types.Response{
		Data: []byte{},
	}
	r := strings.NewReader(*msg)
	w := bytes.NewBuffer(resp.Data)
	i := prolog.New(r, w)
	err := i.QuerySolution(`consult(?).`, 1).Err()
	resp.Log = w.Bytes()
	if err != nil {
		return nil, err
	}
	return resp, nil
}

// Migrate executes queue contract's migration which consists in clearing
// the state and writing three new values in the queue
func Migrate(deps *std.Deps, _ types.Env, _ []byte) (*types.Response, error) {
	iter := deps.Storage.Range(nil, nil, std.Ascending)
	// clear
	for {
		k, _, err := iter.Next()
		if err != nil {
			break
		}
		deps.Storage.Remove(k)
	}
	// add three values
	for i := int32(100); i < 103; i++ {
		_, err := executeEnqueue(deps, types.Env{}, types.MessageInfo{}, &Enqueue{Value: i})
		if err != nil {
			return nil, err
		}
	}
	return &types.Response{}, nil
}

// Query handles given message bytes what query handler must be executed.
func Query(deps *std.Deps, _ types.Env, msg []byte) ([]byte, error) {
	q := new(QueryMsg)
	err := q.UnmarshalJSON(msg)
	if err != nil {
		return nil, err
	}

	switch {
	case q.List != nil:
		return queryList(deps)
	case q.Sum != nil:
		return querySum(deps)
	case q.Reducer != nil:
		return queryReducer(deps)
	case q.Count != nil:
		return queryCount(deps)
	}

	return nil, errors.New("unknown query message") // TODO(fdymylja): use a common error once devex experience on querymsg is sorted.
}

func queryReducer(deps *std.Deps) ([]byte, error) {
	var counters [][2]int32
	iter := deps.Storage.Range(nil, nil, std.Ascending)
	for {
		_, value, err := iter.Next()
		if err != nil {
			break
		}
		item := new(Item)
		if err := item.UnmarshalJSON(value); err != nil {
			return nil, err
		}

		sum := int32(0)
		iter2 := deps.Storage.Range(nil, nil, std.Ascending)
		for {
			_, value2, err2 := iter2.Next()
			if err2 != nil {
				break
			}
			item2 := new(Item)
			if err := item2.UnmarshalJSON(value2); err != nil {
				return nil, err
			}
			// skip second iterator items whose value is lower than the first
			if item2.Value <= item.Value {
				continue
			}
			sum += item2.Value
		}

		counters = append(counters, [2]int32{item.Value, sum})
	}

	resp := &ReducerResponse{Counters: counters}
	return resp.MarshalJSON()
}

func queryList(deps *std.Deps) ([]byte, error) {
	// do empty
	emptyIter := deps.Storage.Range([]byte("large"), []byte("large"), std.Ascending)
	var empty []uint32
	for {
		k, _, err := emptyIter.Next()
		if err != nil {
			break
		}
		empty = append(empty, (uint32)(k[0]))
	}
	// do early
	earlyIter := deps.Storage.Range(nil, []byte{20}, std.Ascending)
	var early []uint32
	for {
		k, _, err := earlyIter.Next()
		if err != nil {
			break
		}
		early = append(early, (uint32)(k[0]))
	}
	// do late
	lateIter := deps.Storage.Range([]byte{20}, nil, std.Ascending)
	var late []uint32
	for {
		k, _, err := lateIter.Next()
		if err != nil {
			break
		}
		late = append(late, (uint32)(k[0]))
	}

	resp := ListResponse{
		Empty: empty,
		Early: early,
		Late:  late,
	}

	b, err := resp.MarshalJSON()
	if err != nil {
		return nil, err
	}

	return b, nil
}

func queryCount(deps *std.Deps) ([]byte, error) {
	iter := deps.Storage.Range(nil, nil, std.Ascending)

	var count uint32
	for {
		_, _, err := iter.Next()
		if err != nil {
			break
		}
		count++
	}

	resp := CountResponse{Count: count}

	b, err := resp.MarshalJSON()
	if err != nil {
		return nil, err
	}

	return b, nil
}

func querySum(deps *std.Deps) ([]byte, error) {
	var sum int32
	iter := deps.Storage.Range(nil, nil, std.Ascending)
	for {
		_, v, err := iter.Next()
		if err != nil {
			break
		}
		item := new(Item)
		err = item.UnmarshalJSON(v)
		if err != nil {
			return nil, err
		}
		sum += item.Value
	}

	resp := SumResponse{Sum: sum}
	b, err := resp.MarshalJSON()
	if err != nil {
		return nil, err
	}

	return b, nil
}
