//
//  HomeView.swift
//  Libellule
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import LibelluleKit

struct HomeView: View {
    var client: Client
    @State private var gradesData: GradesData?

    var body: some View {
        TabView {
            Tab("Emploi du temps", systemImage: "calendar") {
                TimetableView(client: client)
            }
            Tab("Notes", systemImage: "graph.2d") {
                GradesView(client: client)

            }
        }
    }
}
