package libhive_test

import (
	"path/filepath"
	"testing"

	"github.com/ethereum/hive/internal/libhive"
)

func TestSplitClientName(t *testing.T) {
	tests := []struct {
		name                   string
		wantClient, wantBranch string
	}{
		{"client", "client", ""},
		{"client_b", "client", "b"},
		{"the_client_b", "the_client", "b"},
	}
	for _, test := range tests {
		c, b := libhive.SplitClientName(test.name)
		if c != test.wantClient || b != test.wantBranch {
			t.Errorf("SpnlitClientName(%q) -> (%q, %q), want (%q, %q)", test.name, c, b, test.wantClient, test.wantBranch)
		}
	}
}

func TestInventory(t *testing.T) {
	basedir := filepath.FromSlash("../..")
	inv, err := libhive.LoadInventory(basedir)
	if err != nil {
		t.Fatal(err)
	}

	t.Run("HasClient", func(t *testing.T) {
		if !inv.HasClient("trin") {
			t.Error("can't find trin client")
		}
		if inv.HasClient("supereth3000") {
			t.Error("returned true for unknown client")
		}
	})
	t.Run("HasSimulator", func(t *testing.T) {
		if !inv.HasSimulator("history/rpc-compat") {
			t.Error("can't find rpc-compat simulator")
		}
		if inv.HasSimulator("unknown simulator name") {
			t.Error("returned true for unknown simulator name")
		}
	})
}
