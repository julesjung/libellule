//
//  MainView.swift
//  Pronote
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import PronoteKit

struct MainView: View {
    var client: Client
    @State private var gradesData: GradesData?
    
    var body: some View {
        TabView {
            Tab("Emploi du temps", systemImage: "calendar") {
                TimetableView(client: client)
            }
            Tab("Notes", systemImage: "graph.2d") {
                if gradesData == nil {
                    ProgressView("Chargement des notes")
                        .task {
                            let periods = await client.getPeriods()
                            let defaultPeriodId = await client.getDefaultPeriod()
                            let defaultPeriod = periods.first(where: { $0.id == defaultPeriodId })
                            gradesData = try! await client.getGrades(period: defaultPeriod!)
                        }
                } else {
                    GradesView(gradesData: gradesData!)
                }
            }
        }
    }
}
