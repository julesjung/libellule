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
    @State private var temporarySelection: Date
    
    init(selection: Binding<Date>, in: ClosedRange<Date>) {
        self._selection = selection
        self._in = State(initialValue: `in`)
        self._temporarySelection = State(initialValue: selection.wrappedValue)
    }
    
    var body: some View {
        Button {
            temporarySelection = selection
            showingDatePicker = true
        } label: {
            Text(selection.formatted(.dateTime.weekday(.wide).day().month(.wide)).localizedCapitalized)
        }
        .sheet(isPresented: $showingDatePicker) {
            NavigationStack {
                CalendarView(selection: $temporarySelection, in: `in`)
                    .navigationTitle("Date")
                    .navigationBarTitleDisplayMode(.inline)
                    .toolbar {
                        ToolbarItem(placement: .confirmationAction) {
                            Button(role: .confirm) {
                                selection = temporarySelection
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
