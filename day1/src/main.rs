//!
//! [Day 1 Advent of Code 2019] solution in Rust.
//!
//! # Part 1
//! Solution implemented as [`FuelCalculator#calculate_required_fuel`].
//!
//! Not much to explain, the function maps over module masses and returns sum of per-module fuel requirements.
//! Modules of mass `< 6` are ignored as they would cause underflow, although no such modules are present
//! in the input data. In theory, any module with mass `< 8` will require no fuel and therefore could be ignored
//! instead.
//!
//! # Part 2
//! Solution implemented as [`FuelCalculator#calculate_total_fuel`].
//!
//! The algorithm first calculates the module fuer requirements and the iteratively calculates the fuel
//! required to carry previously calculated fuel mass until the required additioal fuel is 0. Then it
//! returs the sum of all the fuel masses it calculated.
//!
//! The tricky part here was to treat each module fuel separatly instead of calculating fuel requirement
//! for the aggregated result from *Part 1*.
//!
//! There also might be a mathematical formula to calculate the total fuel necessary as it seems to be
//! a geometrical series that only depends on initial module mass.
//!
//! [Day 1 Advent of Code 2019]: https://adventofcode.com/2019/day/1
//! [`FuelCalculator#calculate_required_fuel`]: struct.FuelCalculator.html#method.calculate_required_fuel
//! [`FuelCalculator#calculate_total_fuel`]: struct.FuelCalculator.html#method.calculate_total_fuel

use std::borrow::Borrow;
use std::fs::read_to_string;
use std::iter::{once, successors, Once};

/// Calculates fuel required for a module of a given `mass`.
///
/// This can panic due to overflow if mass is below 6.
#[inline]
fn calculate_module_fuel_req(mass: &u32) -> u32 {
    mass / 3 - 2
}

/// Calculates total fuel required for a module of a given `mass`.
///
/// This takes into account fuel required to carry both the module and
/// the fuel mass.
#[inline]
fn calculate_module_total_fuel_req(mass: &u32) -> u32 {
    let module_fuel = calculate_module_fuel_req(mass);
    successors(Some(module_fuel), |f| {
        if *f <= 6 {
            None
        } else {
            Some(calculate_module_fuel_req(f))
        }
    })
    .sum()
}

/// Calculates fuel requirements of a provided modules.
///
/// The main goal is to abstract calculations for a single module and
/// for multiple modules. Uses [`Borrow<u32>`] because the calculations
/// does not depend of the ownership of the contained mass value,
/// so for multiple modules we want to avoid copying/cloning entire
/// iterable content.
///
/// [`Borrow<u32>`]: https://doc.rust-lang.org/std/borrow/trait.Borrow.html
pub struct FuelCalculator<N: Borrow<u32>, T: Iterator<Item = N>> {
    iter: T,
}

impl<'a> FuelCalculator<u32, Once<u32>> {
    /// Creates fuel calculator for a single module with a given `mass`
    pub fn for_module(mass: u32) -> FuelCalculator<u32, Once<u32>> {
        FuelCalculator { iter: once(mass) }
    }
}

impl<'a, T: Iterator<Item = &'a u32>> FuelCalculator<&'a u32, T> {
    /// Creates fuel calculator for all the modules with the masses
    /// in the iterator.
    pub fn for_modules(modules: T) -> FuelCalculator<&'a u32, T> {
        FuelCalculator { iter: modules }
    }
}

impl<N: Borrow<u32>, T: Iterator<Item = N>> FuelCalculator<N, T> {
    /// Calculates fuel required for modules.
    pub fn calculate_required_fuel(self) -> u32 {
        self.iter
            .filter(|m| *m.borrow() > 6)
            .map(|v| calculate_module_fuel_req(v.borrow()))
            .sum()
    }

    /// Calculates total fuel required for modules and the fuel.
    pub fn calculate_total_fuel(self) -> u32 {
        self.iter
            .map(|v| calculate_module_total_fuel_req(v.borrow()))
            .sum()
    }
}

fn read_input() -> Vec<u32> {
    read_to_string(INPUT_PATH)
        .unwrap_or_else(|e| panic!("Error while opening file {}: {}", INPUT_PATH, e))
        .lines()
        .map(|l| {
            l.parse::<u32>()
                .unwrap_or_else(|e| panic!("Unable to convert {} to int32", e))
        })
        .collect()
}

static INPUT_PATH: &str = "day1/data/input.txt";

fn main() {
    let input = read_input();
    let modules_fuel = FuelCalculator::for_modules(input.iter()).calculate_required_fuel();
    println!("Required fuel for modules: {}", modules_fuel);

    let total_fuel = FuelCalculator::for_modules(input.iter()).calculate_total_fuel();
    println!("Required fuel total: {}", total_fuel);
}

#[cfg(test)]
mod test {
    use super::FuelCalculator;

    #[test]
    fn check_calculate_required_fuel() {
        vec![(14, 2), (12, 2), (1969, 654), (100756, 33583)]
            .iter()
            .for_each(|(mass, expected): &(u32, u32)| {
                let calc = FuelCalculator::for_module(*mass);
                assert_eq!(calc.calculate_required_fuel(), *expected);
            });
    }

    #[test]
    fn check_calculate_total_fuel() {
        vec![(14, 2), (12, 2), (1969, 966), (100756, 50346)]
            .iter()
            .for_each(|(mass, expected): &(u32, u32)| {
                let calc = FuelCalculator::for_module(*mass);
                assert_eq!(calc.calculate_total_fuel(), *expected);
            });
    }
}
