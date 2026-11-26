Imports System

Namespace Shop
    Public Interface ICounter
        Function Increment(stepSize As Integer) As Integer
    End Interface

    Public Enum Mode
        Normal
        Fast
    End Enum

    Public Structure Point
        Public X As Integer
        Public Y As Integer
    End Structure

    Public Delegate Sub ChangedHandler(value As Integer)

    Public Class CounterBase
        Protected Overridable Sub Reset()
        End Sub
    End Class

    Public Class Counter
        Public Const MaxCount As Integer = 10
        Private _count As Integer
        Public Event Changed(value As Integer)

        Public Sub New()
            _count = 0
        End Sub

        Public ReadOnly Property Count As Integer
            Get
                Return _count
            End Get
        End Property

        Private Sub OnChanged(value As Integer)
            Console.WriteLine(value)
        End Sub

        Public Function Increment(stepSize As Integer) As Integer
            _count = Math.Min(_count + stepSize, MaxCount)
            OnChanged(_count)
            Return _count
        End Function
    End Class

    Module Program
        Sub Main()
            Dim counter = New Counter()
            Console.WriteLine(counter.Increment(2))
        End Sub
    End Module
End Namespace
