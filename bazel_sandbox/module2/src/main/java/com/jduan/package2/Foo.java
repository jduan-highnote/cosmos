package com.jduan.package2;

import com.google.common.collect.ImmutableList;
import java.util.List;

public class Foo {
  public List<String> getFoos() {
    return ImmutableList.of("foo1", "foo2", "foo3");
  }
}
