//
//  CalendarView.swift
//  Libellule
//
//  Created by Jules on 13/08/2026.
//

import SwiftUI
import UIKit

struct CalendarView: UIViewRepresentable {
    @Binding var selection: Date
    var `in`: ClosedRange<Date>

    func makeCoordinator() -> Coordinator {
        Coordinator(selection: $selection)
    }

    func makeUIView(context: Context) -> UICalendarView {
        let calendarView = UICalendarView()

        calendarView.calendar = Calendar.current
        calendarView.locale = Locale.current
        calendarView.fontDesign = .rounded
        calendarView.availableDateRange = DateInterval(start: `in`.lowerBound, end: `in`.upperBound)

        let selection = UICalendarSelectionSingleDate(
            delegate: context.coordinator
        )

        selection.selectedDate = calendarView.calendar.dateComponents(
            [.year, .month, .day],
            from: self.selection
        )

        calendarView.selectionBehavior = selection

        calendarView.visibleDateComponents = calendarView.calendar.dateComponents(
            [.year, .month, .day],
            from: self.selection
        )

        return calendarView
    }

    func updateUIView(
        _ calendarView: UICalendarView,
        context: Context
    ) {
        let components = calendarView.calendar.dateComponents(
            [.year, .month, .day],
            from: selection
        )

        guard
            let selection = calendarView.selectionBehavior
                as? UICalendarSelectionSingleDate
        else {
            return
        }

        if selection.selectedDate != components {
            selection.setSelected(components, animated: false)
        }
    }
    
    func sizeThatFits(_ proposal: ProposedViewSize, uiView: UICalendarView, context: Context) -> CGSize? {
        let width = proposal.width ?? 0

                return uiView.systemLayoutSizeFitting(
                    CGSize(
                        width: width,
                        height: UIView.layoutFittingCompressedSize.height
                    ),
                    withHorizontalFittingPriority: .required,
                    verticalFittingPriority: .fittingSizeLevel
                )
    }

    final class Coordinator: NSObject, UICalendarSelectionSingleDateDelegate {
        var selection: Binding<Date>

        init(selection: Binding<Date>) {
            self.selection = selection
        }

        func dateSelection(
            _ selection: UICalendarSelectionSingleDate,
            didSelectDate dateComponents: DateComponents?
        ) {
            guard let dateComponents else {
                return
            }

            if let date = dateComponents.date {
                self.selection.wrappedValue = date
            }
        }
    }
}
