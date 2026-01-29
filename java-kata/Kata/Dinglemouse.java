package Kata;

public class Dinglemouse {



    private static int ONE_HUNDRED = 100;

    private final int value;

    private Dinglemouse() {
        value = ONE_HUNDRED;
    }

    public static final Dinglemouse INST = new Dinglemouse();
    public int plus100(int n) {
        return value + n;
    }

}
