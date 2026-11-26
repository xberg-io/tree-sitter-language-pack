Imports System

Namespace Shop
    Public Interface ICounter
        Function Increment(stepSize As Integer) As Integer
    End Interface

    Public Enum Mode
        Normal
        Fast
    End Enum

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
