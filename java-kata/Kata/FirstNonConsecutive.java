package Kata;

public class FirstNonConsecutive {
    static Integer find(final int[] arr) {
        for (int i = 0; i < arr.length - 1; i++)
            if (arr[i+1] - arr[i] != 1)
                return arr[i+1];
        return null;
    }
}
