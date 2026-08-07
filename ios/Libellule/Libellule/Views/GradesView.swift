//
//  GradesView.swift
//  Libellule
//
//  Created by Jules on 20/07/2026.
//

import SwiftUI
import LibelluleKit

struct GradesView: View {
    var client: Client
    @State private var gradesData: GradesData?
    @State private var periods: [Period] = []
    @State private var selectedPeriod: Period?

    var body: some View {
        VStack {
            if selectedPeriod == nil {
                Spacer()
                ProgressView("Chargement des périodes")
                    .task {
                        periods = await client.getPeriods()
                        let defaultPeriodId = await client.getDefaultPeriod()
                        if let period = periods.first(where: { $0.id == defaultPeriodId }) {
                            selectedPeriod = period
                        } else {
                            selectedPeriod = periods[0]
                        }
                    }
                Spacer()
            } else {
                Picker("Période", selection: $selectedPeriod) {
                    ForEach(periods, id: \.id) { period in
                        Text(period.name)
                            .tag(period)
                    }
                    .onChange(of: selectedPeriod) {
                        gradesData = nil
                    }
                }
                .pickerStyle(.menu)
                .buttonStyle(.glass)

                if gradesData == nil {
                    Spacer()
                    ProgressView("Chargement des notes")
                        .task {
                            gradesData = try! await client.getGrades(period: selectedPeriod!)
                        }
                    Spacer()
                } else {
                    List(gradesData!.assignments, id: \.self) { assignment in
                        HStack {
                            VStack(alignment: .leading) {
                                Text(assignment.subject.name).lineLimit(1)
                                Text(assignment.label).font(.caption).lineLimit(1)
                            }
                            Spacer()
                            switch assignment.grade {
                            case .graded(let grade):
                                Text("\(grade)/\(assignment.scale)")
                            case .absent:
                                Text("Absent")
                            default:
                                Text("Non Noté")
                            }
                        }
                    }
                    .listStyle(.plain)
                }
            }
        }
    }
}
