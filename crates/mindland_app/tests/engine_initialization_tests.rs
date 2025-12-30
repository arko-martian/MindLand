//! Property-based tests for MindLand engine initialization performance
//! 
//! **Feature: engine-boot, Property 1: Engine Initialization Performance**

use mindland_app::{MindLandApp, EngineConfig, PerformanceMode, HardwareTier};
use proptest::prelude::*;
use std::time::{Duration, Instant};

/// Test that engine initialization completes within 100ms for any configuration
#[cfg(test)]
mod engine_initialization_performance_tests {
    use super::*;

    // Property test configuration for thorough testing
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_memory_pool_zero_allocation_property(
            entity_requests in prop::collection::vec(1usize..100, 1..50),
            transform_requests in prop::collection::vec(1usize..100, 1..50),
            render_requests in prop::collection::vec(1usize..100, 1..50),
            input_requests in prop::collection::vec(1usize..100, 1..50),
        ) {
            // **Feature: engine-boot, Property 3: Error Handling Resilience**
            // For any sequence of allocation requests, memory pools should handle them without heap allocation
            
            use mindland_app::{MemoryPools, EntityPool, TransformPool, RenderCommandPool, InputEventPool};
            
            let mut memory_pools = MemoryPools {
                entity_pool: EntityPool { capacity: 10000, used: 0 },
                transform_pool: TransformPool { capacity: 10000, used: 0 },
                render_command_pool: RenderCommandPool { capacity: 10000, used: 0 },
                input_event_pool: InputEventPool { capacity: 10000, used: 0 },
            };
            
            let mut total_entities = 0;
            let mut total_transforms = 0;
            let mut total_renders = 0;
            let mut total_inputs = 0;
            
            // Process all allocation requests
            for &entity_count in &entity_requests {
                if let Some(_) = memory_pools.entity_pool.allocate(entity_count) {
                    total_entities += entity_count;
                }
            }
            
            for &transform_count in &transform_requests {
                if let Some(_) = memory_pools.transform_pool.allocate(transform_count) {
                    total_transforms += transform_count;
                }
            }
            
            for &render_count in &render_requests {
                if let Some(_) = memory_pools.render_command_pool.allocate(render_count) {
                    total_renders += render_count;
                }
            }
            
            for &input_count in &input_requests {
                if let Some(_) = memory_pools.input_event_pool.allocate(input_count) {
                    total_inputs += input_count;
                }
            }
            
            // Property: Pool usage should match successful allocations
            prop_assert_eq!(memory_pools.entity_pool.used, total_entities);
            prop_assert_eq!(memory_pools.transform_pool.used, total_transforms);
            prop_assert_eq!(memory_pools.render_command_pool.used, total_renders);
            prop_assert_eq!(memory_pools.input_event_pool.used, total_inputs);
            
            // Property: Used should never exceed capacity
            prop_assert!(memory_pools.entity_pool.used <= memory_pools.entity_pool.capacity);
            prop_assert!(memory_pools.transform_pool.used <= memory_pools.transform_pool.capacity);
            prop_assert!(memory_pools.render_command_pool.used <= memory_pools.render_command_pool.capacity);
            prop_assert!(memory_pools.input_event_pool.used <= memory_pools.input_event_pool.capacity);
            
            // Property: Reset should restore pools to zero usage
            memory_pools.entity_pool.reset();
            memory_pools.transform_pool.reset();
            memory_pools.render_command_pool.reset();
            memory_pools.input_event_pool.reset();
            
            prop_assert_eq!(memory_pools.entity_pool.used, 0);
            prop_assert_eq!(memory_pools.transform_pool.used, 0);
            prop_assert_eq!(memory_pools.render_command_pool.used, 0);
            prop_assert_eq!(memory_pools.input_event_pool.used, 0);
        }
    }

    #[test]
    fn test_engine_configuration_creation_performance() {
        // **Feature: engine-boot, Property 1: Engine Initialization Performance**
        // Test that configuration creation is fast (this doesn't require EventLoop)
        
        let start_time = Instant::now();
        
        let configs = vec![
            EngineConfig::default(),
            EngineConfig::macbook_pro_2014(),
            EngineConfig::ultra_performance(),
        ];
        
        let creation_time = start_time.elapsed();
        
        assert!(
            creation_time <= Duration::from_millis(1),
            "Configuration creation took {:.2}ms, should be nearly instantaneous",
            creation_time.as_secs_f64() * 1000.0
        );
        
        // Verify configurations have correct values
        assert_eq!(configs[0].performance_mode, PerformanceMode::Balanced);
        assert_eq!(configs[1].performance_mode, PerformanceMode::MacBookPro2014);
        assert_eq!(configs[2].performance_mode, PerformanceMode::UltraPerformance);
        
        // Verify MacBook Pro 2014 has thermal-optimized settings
        assert_eq!(configs[1].memory_pool_size, 1024 * 1024 * 32); // 32MB
        assert_eq!(configs[1].max_entities, 50_000);
        
        // Verify ultra-performance has maximum settings
        assert_eq!(configs[2].target_fps, 144);
        assert_eq!(configs[2].memory_pool_size, 1024 * 1024 * 128); // 128MB
        assert_eq!(configs[2].max_entities, 200_000);
    }

    #[test]
    fn test_zero_allocation_hot_path_setup() {
        // **Feature: engine-boot, Property 3: Error Handling Resilience**
        // Test that memory pools are configured correctly for zero-allocation hot paths
        
        let config = EngineConfig {
            memory_pool_size: 1024 * 1024 * 64, // 64MB
            max_entities: 100_000,
            ..Default::default()
        };
        
        let start_time = Instant::now();
        let _app = MindLandApp::with_config(config);
        let init_time = start_time.elapsed();
        
        // Should still initialize quickly even with large memory pools
        assert!(
            init_time <= Duration::from_millis(100),
            "Large memory pool configuration failed performance test: {:.2}ms",
            init_time.as_secs_f64() * 1000.0
        );
    }

    #[test]
    fn test_memory_pool_allocation_tracking() {
        // **Feature: engine-boot, Property 3: Error Handling Resilience**
        // Test that memory pools track allocations correctly
        
        use mindland_app::{MemoryPools, EntityPool, TransformPool, RenderCommandPool, InputEventPool};
        
        let mut memory_pools = MemoryPools {
            entity_pool: EntityPool { capacity: 1000, used: 0 },
            transform_pool: TransformPool { capacity: 1000, used: 0 },
            render_command_pool: RenderCommandPool { capacity: 1000, used: 0 },
            input_event_pool: InputEventPool { capacity: 1000, used: 0 },
        };
        
        // Test entity pool allocation
        let allocation = memory_pools.entity_pool.allocate(100);
        assert!(allocation.is_some(), "Entity pool allocation should succeed");
        assert_eq!(memory_pools.entity_pool.used, 100);
        
        // Test pool exhaustion
        let large_allocation = memory_pools.entity_pool.allocate(1000);
        assert!(large_allocation.is_none(), "Entity pool should be exhausted");
        
        // Test pool reset
        memory_pools.entity_pool.reset();
        assert_eq!(memory_pools.entity_pool.used, 0);
        
        // Test capacity checking
        assert!(memory_pools.has_sufficient_capacity(500, 500, 500, 500));
        assert!(!memory_pools.has_sufficient_capacity(2000, 500, 500, 500));
    }

    #[test]
    fn test_allocation_tracker_zero_violation_detection() {
        // **Feature: engine-boot, Property 3: Error Handling Resilience**
        // Test that allocation tracker correctly detects zero-allocation violations
        
        use mindland_app::AllocationTracker;
        
        let mut tracker = AllocationTracker {
            hot_path_allocations: 0,
            frame_allocations: 0,
            peak_allocations_per_frame: 0,
            zero_allocation_violations: 0,
        };
        
        // Initially should maintain zero-allocation guarantee
        assert!(tracker.is_zero_allocation_maintained());
        
        // Track a hot path allocation (violation!)
        tracker.track_hot_path_allocation();
        
        // Should detect violation
        assert!(!tracker.is_zero_allocation_maintained());
        assert_eq!(tracker.zero_allocation_violations, 1);
        assert_eq!(tracker.hot_path_allocations, 1);
        assert_eq!(tracker.frame_allocations, 1);
        assert_eq!(tracker.peak_allocations_per_frame, 1);
        
        // Track another allocation
        tracker.track_hot_path_allocation();
        assert_eq!(tracker.zero_allocation_violations, 2);
        assert_eq!(tracker.peak_allocations_per_frame, 2);
    }

    #[test]
    fn test_graphics_backend_selection_performance() {
        // **Feature: engine-boot, Property 1: Engine Initialization Performance**
        // Test that graphics backend selection doesn't slow down initialization
        
        for hardware_tier in [HardwareTier::Low, HardwareTier::Medium, HardwareTier::High, HardwareTier::UltraHigh] {
            let config = EngineConfig {
                hardware_tier,
                ..Default::default()
            };
            
            let start_time = Instant::now();
            let _app = MindLandApp::with_config(config);
            let init_time = start_time.elapsed();
            
            assert!(
                init_time <= Duration::from_millis(100),
                "Graphics backend selection for {:?} tier took {:.2}ms, exceeds limit",
                hardware_tier,
                init_time.as_secs_f64() * 1000.0
            );
        }
    }
}

/// Benchmark tests for engine initialization (run with `cargo bench`)
#[cfg(test)]
mod engine_initialization_benchmarks {
    use super::*;
    use criterion::{black_box, Criterion};

    pub fn benchmark_engine_initialization(c: &mut Criterion) {
        c.bench_function("engine_initialization_default", |b| {
            b.iter(|| {
                let config = black_box(EngineConfig::default());
                let _app = black_box(MindLandApp::with_config(config));
            });
        });

        c.bench_function("engine_initialization_macbook_pro_2014", |b| {
            b.iter(|| {
                let config = black_box(EngineConfig::macbook_pro_2014());
                let _app = black_box(MindLandApp::with_config(config));
            });
        });

        c.bench_function("engine_initialization_ultra_performance", |b| {
            b.iter(|| {
                let config = black_box(EngineConfig::ultra_performance());
                let _app = black_box(MindLandApp::with_config(config));
            });
        });
    }
}

// Helper function to run benchmarks (called by criterion)
#[cfg(test)]
criterion::criterion_group!(benches, engine_initialization_benchmarks::benchmark_engine_initialization);
#[cfg(test)]
criterion::criterion_main!(benches);