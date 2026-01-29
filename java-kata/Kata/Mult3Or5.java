package Kata;

public class Mult3Or5 {
    public int solution(int n) {
        int res = 0;
        for (int i = 0; i < n; i++) {
            if (i % 3 == 0 || i % 5 == 0)
                res += i;
        }
        return res;
    }
}
