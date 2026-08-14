//
//  DateSelector.swift
//  Libellule
//
//  Created by Jules on 14/08/2026.
//

import SwiftUI

struct DateSelector: View {
    @Binding var selection: Date
    @State var `in`: ClosedRange<Date>
    
    @State private var showingDatePicker = false
    
    var body: some View {
        Button {
            showingDatePicker = true
        } label: {
            HStack {
                Text(selection.formatted(.dateTime.weekday(.wide)).localizedCapitalized)
                    .font(.headline)
                Text(selection.formatted(.dateTime.day().month(.wide)).localizedCapitalized)
            }
        }
        .sheet(isPresented: $showingDatePicker) {
            NavigationStack {
                CalendarView(selection: $selection, in: `in`)
                    .navigationTitle("Date")
                    .navigationBarTitleDisplayMode(.inline)
                    .toolbar {
                        ToolbarItem(placement: .confirmationAction) {
                            Button(role: .confirm) {
                                showingDatePicker = false
                            }
                        }
                    }
                    .padding()
            }
            .presentationDetents([.medium])
        }
    }
}
