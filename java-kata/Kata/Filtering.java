package Kata;

import java.util.List;

public class Filtering {
    public static List<Object> filterList(final List<Object> l) {
        return l.stream().filter(e -> !(e instanceof String)).toList();
    }
}
