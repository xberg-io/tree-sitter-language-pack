Var OrderCount Export;

Procedure AddOrder(Order) Export
	OrderCount = OrderCount + 1;
	LogOrder(Order);
EndProcedure

Function TotalAmount(Orders)
	Total = 0;
	For Each Order In Orders Do
		Total = Total + Order.Amount;
	EndDo;
	Return Total;
EndFunction

Procedure LogOrder(Order)
	Message(Order.Describe());
EndProcedure

Function FormatOrder(Order) Export
	Return StrTemplate("%1: %2", Order.Number, TotalAmount(Order.Lines));
EndFunction
