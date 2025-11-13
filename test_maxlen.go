package main

import (
	"fmt"
	"math/bits"
)

func literalExtraSize(n int64) int64 {
	if n == 0 {
		return 0
	}
	switch {
	case n < 60:
		return 1
	case n < 1<<8:
		return 2
	case n < 1<<16:
		return 3
	case n < 1<<24:
		return 4
	default:
		return 5
	}
}

func MaxEncodedLen(srcLen int) int {
	n := uint64(srcLen)
	// Size of the varint encoded block size.
	n = n + uint64((bits.Len64(n)+7)/7)
	// Add maximum size of encoding block as literals.
	n += uint64(literalExtraSize(int64(srcLen)))
	return int(n)
}

func main() {
	for _, size := range []int{2166, 2167, 2168, 2169} {
		maxLen := MaxEncodedLen(size)
		bitsNeeded := bits.Len64(uint64(size))
		varintExtra := (bitsNeeded + 7) / 7
		litExtra := literalExtraSize(int64(size))
		fmt.Printf("size=%d: max_len=%d, bits=%d, varint_extra=%d, lit_extra=%d\n",
			size, maxLen, bitsNeeded, varintExtra, litExtra)
	}
}
