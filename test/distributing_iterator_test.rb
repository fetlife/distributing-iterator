# frozen_string_literal: true

require_relative "test_helper"

class DistributingIteratorTest < Minitest::Test
  def test_distribute_csv
    csv = <<~CSV
      id,name
      1,foo
      1,bar
      1,baz
      2,qux
      3,quux
      3,corge
      2,grault
      2,garply
      3,waldo
      3,fred
      2,plugh
      3,xyzzy
      2,thud
      3,plugh
      3,xyzzy
    CSV

    result = DistributingIterator.distribute_csv(csv, "id", 3)

    assert_equal(
      "id,name\n1,foo\n2,qux\n3,quux\n1,bar\n2,grault\n3,corge\n1,baz\n2,garply\n3,waldo\n2,plugh\n3,fred\n2,thud\n3,xyzzy\n3,plugh\n3,xyzzy\n",
      result
    )
  end

  def test_distribute_indexes
    result = DistributingIterator.distribute_indexes(
      %w[Picture Post Video Video Picture Post Picture Picture Video],
      3
    )

    assert_equal [0, 1, 2, 4, 5, 3, 6, 8, 7], result
  end
end
