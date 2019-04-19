package se.hof.adventofcode.eighteen;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class DayEightTest {

    DayEight.Node node;

    @Test
    public void parseChildlessNodeTest() {
        String s = "0 1 99";
        DayEight.Node n = DayEight.Node.parse(s);

        assertEquals(99, n.sum());
    }

    @Test
    public void parseChildlessNodeTest2() {
        String s = "0 3 10 11 12";
        DayEight.Node n = DayEight.Node.parse(s);

        assertEquals(33, n.sum());
    }


    @Test
    public void parseNodeWithChild() {
        String s = "1 1 0 1 99 2";
        DayEight.Node n = DayEight.Node.parse(s);

        assertEquals(101, n.sum());
    }
}
