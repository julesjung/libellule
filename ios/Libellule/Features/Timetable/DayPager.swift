//
//  DayPager.swift
//  Libellule
//
//  Created by Jules on 15/08/2026.
//

import SwiftUI

final class PagingScrollView: UIScrollView {
    var initialPage = 1
    var didSetInitialPosition = false

    override func layoutSubviews() {
        super.layoutSubviews()

        guard !didSetInitialPosition else { return }
        guard bounds.width > 0 else { return }

        didSetInitialPosition = true

        setContentOffset(
            CGPoint(
                x: bounds.width * CGFloat(initialPage),
                y: 0
            ),
            animated: false
        )
    }
}

struct DayPager<Content: View>: UIViewRepresentable {
    let content: Content
    
    let previous: () -> Void
    let next: () -> Void
    
    init(
        previous: @escaping () -> Void,
        next: @escaping () -> Void,
        @ViewBuilder content: () -> Content,
    ) {
        self.content = content()
        self.previous = previous
        self.next = next
    }
    
    func makeUIView(context: Context) -> UIScrollView {
        let scrollView = PagingScrollView()

        scrollView.isPagingEnabled = true
        scrollView.showsHorizontalScrollIndicator = false
        scrollView.alwaysBounceVertical = false
        scrollView.contentInsetAdjustmentBehavior = .never
        scrollView.delegate = context.coordinator

        let hostingController = UIHostingController(rootView: content)

        hostingController.view.translatesAutoresizingMaskIntoConstraints = false

        scrollView.addSubview(hostingController.view)

        NSLayoutConstraint.activate([
            hostingController.view.leadingAnchor
                .constraint(equalTo: scrollView.contentLayoutGuide.leadingAnchor),

            hostingController.view.trailingAnchor
                .constraint(equalTo: scrollView.contentLayoutGuide.trailingAnchor),

            hostingController.view.topAnchor
                .constraint(equalTo: scrollView.contentLayoutGuide.topAnchor),

            hostingController.view.bottomAnchor
                .constraint(equalTo: scrollView.contentLayoutGuide.bottomAnchor),

            hostingController.view.heightAnchor
                .constraint(equalTo: scrollView.frameLayoutGuide.heightAnchor)
        ])

        return scrollView
    }

    func updateUIView(
        _ scrollView: UIScrollView,
        context: Context
    ) {
    }
    
    func makeCoordinator() -> Coordinator {
        Coordinator(previous: previous, next: next)
    }
    
    final class Coordinator: NSObject, UIScrollViewDelegate {
        let previous: () -> Void
        let next: () -> Void
                
        init(previous: @escaping () -> Void, next: @escaping () -> Void) {
            self.previous = previous
            self.next = next
        }
        
        func scrollViewDidEndDecelerating(_ scrollView: UIScrollView) {
            let page = Int(
                round(scrollView.contentOffset.x / scrollView.bounds.width)
            )

            switch page {
            case 0:
                previous()

            case 2:
                next()

            default:
                break
            }
        }
    }
}
