//
//  RootView.swift
//  Libellule
//
//  Created by Jules on 10/08/2026.
//

import SwiftUI

struct RootView: View {
    @State var appParameters: AppParameters?
    @State var currentTab: AppTab = .home
    
    init(appParameters: AppParameters?) {
        self._appParameters = State(initialValue: appParameters)
    }
    
    var body: some View {
        if let appParameters {
            tabs(appParameters: appParameters)
        } else {
            LoginView(appParameters: $appParameters)
        }
    }
    
    @ViewBuilder
    func tabs(appParameters: AppParameters) -> some View {
        let datesRange: ClosedRange<Date> = {
            let startDate = FFIDate.date.date(from: appParameters.startDate)!
            let endDate = FFIDate.date.date(from: appParameters.endDate)!
            
            return startDate...endDate
        }()
        
        TabView(selection: $currentTab) {
            Tab("Accueil", systemImage: "house", value: .home) {
                HomeView()
            }
            
            Tab("EDT", systemImage: "calendar.day.timeline.left", value: .timetable) {
                TimetableView(datesRange: datesRange)
            }
            
            Tab("Devoirs", systemImage: "checkmark.square", value: .homework) {
                HomeworkView(datesRange: datesRange)
            }
        }
    }
}

enum AppTab {
    case home, timetable, homework
}
