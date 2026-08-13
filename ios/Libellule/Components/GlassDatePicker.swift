//
//  GlassDatePicker.swift
//  Libellule
//
//  Created by Jules on 12/08/2026.
//

import SwiftUI

struct GlassDatePicker: View {
    @Binding var selection: Date
    let `in`: ClosedRange<Date>?

    var body: some View {
        Button {
            
        } label: {
            Text(selection, format: .dateTime
                .day()
                .month()
                .year()
            )
        }
        .buttonStyle(.glass)
        .overlay {
            DatePicker(
                "Date",
                selection: $selection,
                in: self.in ?? Date.distantPast...Date.distantFuture,
                displayedComponents: [.date]
            )
            .labelsHidden()
            .colorMultiply(.clear)
        }
    }
}
