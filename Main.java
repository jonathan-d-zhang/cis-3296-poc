public class Main {
    public static void main(String[] args) {
        System.out.println(add(4, 1));
        OtherClass.method();
    }

    public static int add(int a, int b) {
        return a + b;
    }
}


class OtherClass extends Main {
    protected static void method() {
        System.out.println("OtherClass.method");
    }
}
