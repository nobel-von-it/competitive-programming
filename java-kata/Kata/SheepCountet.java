package Kata;

import java.util.Objects;
import java.util.stream.Stream;

public class SheepCountet {
    public static int countSheeps(Boolean[] a) {
        return a != null ? Stream.of(a).filter(Objects::nonNull).filter(Boolean::booleanValue).toArray().length : 0;
    }
    public static int countSheepsFast(Boolean[] a) {
        int count = 0;
        for (Boolean b: a)
            if (b != null && b)
                count++;
        return count;
    }
}
