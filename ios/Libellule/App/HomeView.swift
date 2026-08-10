//
//  HomeView.swift
//  Libellule
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import LibelluleKit

struct HomeView: View {
    let client: Client

    var body: some View {
        TabView {
            /*
            Tab("Emploi du temps", systemImage: "calendar") {
                TimetableView()
            }
             */
            Tab("Notes", systemImage: "graph.2d") {
                GradesView(store: GradesStore(client: client))

            }
        }
    }
}
