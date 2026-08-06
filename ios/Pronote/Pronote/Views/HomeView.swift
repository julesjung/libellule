//
//  HomeView.swift
//  Pronote
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import PronoteKit

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
