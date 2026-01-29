package Kata;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.IntStream;

public class ZywOo {
    public static int[] multipleOfIndex(int[] arr) {
        List<Integer> tmp = new ArrayList<>();

        if (arr[0] == 0)
            tmp.add(0);
        for (int i = 1; i < arr.length; i++) {
            if (arr[i] % i == 0) {
                tmp.add(arr[i]);
            }
        }

        int[] res = new int[tmp.size()];
        for (int i = 0; i < res.length; i++) {
            res[i] = tmp.get(i);
        }
        return res;
    }

    //or minimum code
    public static int[] multInd(int[] arr) {
        return IntStream.range(0, arr.length).filter(i -> {
            if (i == 0)
                return arr[i] == 0;
            return arr[i] % i == 0;
        }).map(i -> arr[i]).toArray();
    }
}
